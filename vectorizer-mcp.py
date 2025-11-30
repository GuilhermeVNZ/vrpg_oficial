#!/usr/bin/env python3
"""
Vectorizer MCP Bridge Server
Bridge for Vectorizer MCP server (port 15002)
"""

import asyncio
import json
import sys
from typing import Any, Dict
import aiohttp

VECTORIZER_BASE_URL = "http://127.0.0.1:15002"

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
                    "name": "vectorizer",
                    "version": "0.9.0"
                }
            }
        }
    
    elif method == "tools/list":
        tools = [
            {
                "name": "vectorizer_search_vectors",
                "description": "Perform semantic search across vectors in a collection",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "collection": {"type": "string", "description": "Collection name"},
                        "query": {"type": "string", "description": "Search query"},
                        "limit": {"type": "integer", "description": "Maximum results (default: 10)"}
                    },
                    "required": ["collection", "query"]
                }
            },
            {
                "name": "vectorizer_intelligent_search",
                "description": "Advanced multi-query search with semantic reranking",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "Search query"},
                        "collections": {"type": "array", "items": {"type": "string"}, "description": "Collection names (empty = all)"},
                        "max_results": {"type": "integer", "description": "Maximum results (default: 5)"}
                    },
                    "required": ["query"]
                }
            },
            {
                "name": "vectorizer_list_collections",
                "description": "List all available collections",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "vectorizer_insert_texts",
                "description": "Insert texts into a collection with automatic embedding",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "collection": {"type": "string", "description": "Collection name"},
                        "vectors": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "id": {"type": "string"},
                                    "text": {"type": "string"},
                                    "metadata": {"type": "object"}
                                },
                                "required": ["id", "text"]
                            },
                            "description": "Array of text vectors to insert"
                        }
                    },
                    "required": ["collection", "vectors"]
                }
            },
            {
                "name": "vectorizer_get_collection_info",
                "description": "Get detailed information about a collection",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "collection": {"type": "string", "description": "Collection name"}
                    },
                    "required": ["collection"]
                }
            },
            {
                "name": "vectorizer_health_check",
                "description": "Check Vectorizer server health",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
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
        
        # Call Vectorizer via HTTP
        async with aiohttp.ClientSession() as session:
            try:
                # Map tool names to Vectorizer API endpoints and HTTP methods
                if tool_name == "vectorizer_list_collections":
                    # GET /collections
                    async with session.get(
                        f"{VECTORIZER_BASE_URL}/collections",
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
                                    "message": f"Vectorizer HTTP error {response.status}: {error_text}"
                                }
                            }
                
                elif tool_name == "vectorizer_get_collection_info":
                    # GET /collections/{collection}
                    collection = arguments.get("collection")
                    if not collection:
                        return {
                            "jsonrpc": "2.0",
                            "id": request_id,
                            "error": {
                                "code": -32602,
                                "message": "Missing required parameter: collection"
                            }
                        }
                    async with session.get(
                        f"{VECTORIZER_BASE_URL}/collections/{collection}",
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
                                    "message": f"Vectorizer HTTP error {response.status}: {error_text}"
                                }
                            }
                
                elif tool_name == "vectorizer_search_vectors":
                    # POST /collections/{collection}/search/text
                    collection = arguments.get("collection")
                    query = arguments.get("query")
                    limit = arguments.get("limit", 10)
                    
                    if not collection or not query:
                        return {
                            "jsonrpc": "2.0",
                            "id": request_id,
                            "error": {
                                "code": -32602,
                                "message": "Missing required parameters: collection and query"
                            }
                        }
                    
                    search_payload = {
                        "query": query,
                        "limit": limit
                    }
                    
                    async with session.post(
                        f"{VECTORIZER_BASE_URL}/collections/{collection}/search/text",
                        json=search_payload,
                        headers={"Content-Type": "application/json"},
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
                                    "message": f"Vectorizer HTTP error {response.status}: {error_text}"
                                }
                            }
                
                elif tool_name == "vectorizer_insert_texts":
                    # POST /collections/{collection}/batch/insert
                    collection = arguments.get("collection")
                    vectors = arguments.get("vectors", [])
                    
                    if not collection or not vectors:
                        return {
                            "jsonrpc": "2.0",
                            "id": request_id,
                            "error": {
                                "code": -32602,
                                "message": "Missing required parameters: collection and vectors"
                            }
                        }
                    
                    # Convert vectors format to Vectorizer API format
                    # API espera "texts" não "vectors"
                    texts_payload = {
                        "texts": []
                    }
                    for vec in vectors:
                        text_data = {
                            "id": vec.get("id"),
                            "text": vec.get("text")
                        }
                        if "metadata" in vec:
                            text_data["metadata"] = vec["metadata"]
                        texts_payload["texts"].append(text_data)
                    
                    # Tentar primeiro com /batch/insert que é mais confiável
                    async with session.post(
                        f"{VECTORIZER_BASE_URL}/collections/{collection}/batch/insert",
                        json=texts_payload,
                        headers={"Content-Type": "application/json"},
                        timeout=aiohttp.ClientTimeout(total=60)
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
                                    "message": f"Vectorizer HTTP error {response.status}: {error_text}"
                                }
                            }
                
                elif tool_name == "vectorizer_health_check":
                    # GET /health
                    async with session.get(
                        f"{VECTORIZER_BASE_URL}/health",
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
                                    "message": f"Vectorizer HTTP error {response.status}: {error_text}"
                                }
                            }
                
                elif tool_name == "vectorizer_intelligent_search":
                    # This would need a specific endpoint - for now, fall back to regular search
                    return {
                        "jsonrpc": "2.0",
                        "id": request_id,
                        "error": {
                            "code": -32601,
                            "message": "Intelligent search not yet implemented in this bridge"
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
                
            except Exception as e:
                # Fallback error handling
                async with session.post(
                    f"{VECTORIZER_BASE_URL}/health",
                    timeout=aiohttp.ClientTimeout(total=5)
                ) as health_check:
                    if health_check.status != 200:
                        return {
                            "jsonrpc": "2.0",
                            "id": request_id,
                            "error": {
                                "code": -32603,
                                "message": f"Vectorizer server not accessible. Health check failed: {str(e)}"
                            }
                        }
                
                return {
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "error": {
                        "code": -32603,
                        "message": f"Failed to call Vectorizer: {str(e)}"
                    }
                }
            
            except asyncio.TimeoutError:
                return {
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "error": {
                        "code": -32603,
                        "message": "Vectorizer request timeout"
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





