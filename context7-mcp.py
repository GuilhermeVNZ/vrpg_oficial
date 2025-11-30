#!/usr/bin/env python3
"""
Context7 MCP Bridge Server
Bridge for Context7 MCP service for library documentation access
"""

import asyncio
import json
import sys
import os
from typing import Any, Dict
import aiohttp

# Context7 API configuration
CONTEXT7_API_URL = os.getenv("CONTEXT7_API_URL", "https://mcp.context7.com/mcp")
CONTEXT7_API_KEY = os.getenv("CONTEXT7_API_KEY", "")

async def handle_request(request: Dict[str, Any]) -> Dict[str, Any]:
    """Handle MCP request"""
    method = request.get("method")
    params = request.get("params", {})
    request_id = request.get("id")
    
    if method == "initialize":
        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "context7",
                    "version": "1.0.0"
                }
            }
        }
    
    elif method == "tools/list":
        tools = [
            {
                "name": "context7_resolve_library_id",
                "description": "Resolve a package name to Context7-compatible library ID",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "library_name": {
                            "type": "string",
                            "description": "Package/library name (e.g., 'tokio', 'react')"
                        }
                    },
                    "required": ["library_name"]
                }
            },
            {
                "name": "context7_get_library_docs",
                "description": "Fetch library documentation with optional topic filter",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "library_id": {
                            "type": "string",
                            "description": "Context7 library ID (e.g., '/tokio-rs/tokio')"
                        },
                        "topic": {
                            "type": "string",
                            "description": "Optional topic filter (e.g., 'async', 'http')"
                        }
                    },
                    "required": ["library_id"]
                }
            }
        ]
        
        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "result": {
                "tools": tools
            }
        }
    
    elif method == "tools/call":
        tool_name = params.get("name")
        arguments = params.get("arguments", {})
        
        # Build headers
        headers = {
            "Content-Type": "application/json"
        }
        if CONTEXT7_API_KEY:
            headers["CONTEXT7_API_KEY"] = CONTEXT7_API_KEY
        
        # Call Context7 API
        async with aiohttp.ClientSession() as session:
            try:
                if tool_name == "context7_resolve_library_id":
                    library_name = arguments.get("library_name")
                    payload = {
                        "method": "resolve_library_id",
                        "params": {"library_name": library_name}
                    }
                elif tool_name == "context7_get_library_docs":
                    library_id = arguments.get("library_id")
                    topic = arguments.get("topic")
                    payload = {
                        "method": "get_library_docs",
                        "params": {
                            "library_id": library_id,
                            **({"topic": topic} if topic else {})
                        }
                    }
                else:
                    return {
                        "jsonrpc": "2.0",
                        "id": request_id,
                        "error": {
                            "code": -32601,
                            "message": f"Unknown tool: {tool_name}"
                        }
                    }
                
                async with session.post(
                    CONTEXT7_API_URL,
                    json=payload,
                    headers=headers,
                    timeout=aiohttp.ClientTimeout(total=30)
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        return {
                            "jsonrpc": "2.0",
                            "id": request_id,
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": json.dumps(result, indent=2, ensure_ascii=False)
                                    }
                                ]
                            }
                        }
                    else:
                        error_text = await response.text()
                        return {
                            "jsonrpc": "2.0",
                            "id": request_id,
                            "error": {
                                "code": -32603,
                                "message": f"Context7 HTTP error {response.status}: {error_text}"
                            }
                        }
            except asyncio.TimeoutError:
                return {
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "error": {
                        "code": -32603,
                        "message": "Context7 request timeout"
                    }
                }
            except Exception as e:
                # Fallback: Return helpful message if Context7 is not available
                return {
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": json.dumps({
                                    "message": f"Context7 service unavailable: {str(e)}",
                                    "note": "Context7 provides library documentation access. Configure CONTEXT7_API_KEY environment variable if you have access."
                                }, indent=2, ensure_ascii=False)
                            }
                        ]
                    }
                }
    
    else:
        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "error": {
                "code": -32601,
                "message": f"Method not found: {method}"
            }
        }

async def main():
    """Main server loop"""
    while True:
        try:
            line = await asyncio.get_event_loop().run_in_executor(None, sys.stdin.readline)
            if not line:
                break
                
            line = line.strip()
            if not line:
                continue
            
            # Parse JSON request
            try:
                request = json.loads(line)
            except json.JSONDecodeError as e:
                # Invalid JSON - try to extract id if possible, otherwise skip
                try:
                    # Try partial parse to get id
                    partial = json.loads(line[:200])  # First 200 chars
                    req_id = partial.get("id") if isinstance(partial, dict) else None
                    if req_id is not None:
                        error_response = {
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "error": {
                                "code": -32700,
                                "message": f"Parse error: {str(e)}"
                            }
                        }
                        print(json.dumps(error_response), flush=True)
                        sys.stdout.flush()
                except:
                    # Can't parse at all, skip this line silently
                    pass
                continue
            
            # Validate request has required fields
            if not isinstance(request, dict) or "method" not in request:
                continue
            
            # Process valid request
            response = await handle_request(request)
            
            # Only send response if we have a valid ID
            if response and response.get("id") is not None:
                print(json.dumps(response), flush=True)
                sys.stdout.flush()
            
        except Exception as e:
            # Only send error if we have request context
            try:
                if 'request' in locals() and isinstance(request, dict):
                    req_id = request.get("id")
                    if req_id is not None:
                        error_response = {
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "error": {
                                "code": -32603,
                                "message": f"Internal error: {str(e)}"
                            }
                        }
                        print(json.dumps(error_response), flush=True)
                        sys.stdout.flush()
            except:
                # Silently ignore errors during error handling
                pass

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        pass
    except Exception as e:
        # Don't send error response on fatal errors, just exit silently
        sys.exit(1)

if __name__ == "__main__":
    asyncio.run(main())





