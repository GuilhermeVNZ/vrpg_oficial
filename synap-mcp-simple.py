#!/usr/bin/env python3
"""
Simple Synap MCP Bridge Server
Minimal implementation for Cursor integration
"""

import asyncio
import json
import sys
from typing import Any, Dict
import aiohttp

SYNAP_BASE_URL = "http://127.0.0.1:15500"

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
                    "name": "synap",
                    "version": "0.1.0"
                }
            }
        }
    
    elif method == "tools/list":
        tools = [
            {
                "name": "synap_kv_get",
                "description": "Retrieve a value from Synap key-value store",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "key": {"type": "string", "description": "The key to retrieve"}
                    },
                    "required": ["key"]
                }
            },
            {
                "name": "synap_kv_set",
                "description": "Store a value in Synap key-value store", 
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "key": {"type": "string", "description": "The key to store"},
                        "value": {"type": "string", "description": "The value to store"},
                        "ttl": {"type": "integer", "description": "Time to live in seconds (optional)"}
                    },
                    "required": ["key", "value"]
                }
            },
            {
                "name": "synap_queue_publish",
                "description": "Publish a message to Synap queue",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "queue": {"type": "string", "description": "Queue name"},
                        "message": {"type": "string", "description": "Message content"},
                        "priority": {"type": "integer", "description": "Message priority 0-9 (optional)"}
                    },
                    "required": ["queue", "message"]
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
        
        # Call Synap via HTTP
        async with aiohttp.ClientSession() as session:
            payload = {
                "method": "tools/call",
                "params": {
                    "name": tool_name,
                    "arguments": arguments
                }
            }
            
            try:
                async with session.post(
                    f"{SYNAP_BASE_URL}/mcp",
                    json=payload,
                    headers={"Content-Type": "application/json"}
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
                                        "text": json.dumps(result.get("result", {}), indent=2)
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
                                "message": f"Synap HTTP error {response.status}: {error_text}"
                            }
                        }
            except Exception as e:
                return {
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "error": {
                        "code": -32603,
                        "message": f"Failed to call Synap: {str(e)}"
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










