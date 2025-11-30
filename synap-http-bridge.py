#!/usr/bin/env python3
"""
Synap HTTP Bridge Server
Simula o servidor Synap na porta 15500 mas redireciona para MCP tools
"""

import asyncio
import json
from aiohttp import web, ClientSession
import logging

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Synap server configuration
SYNAP_BASE_URL = "http://127.0.0.1:15501"  # Real Synap on different port

class SynapHTTPBridge:
    def __init__(self):
        self.session = None
        
    async def init_session(self):
        if not self.session:
            self.session = ClientSession()
    
    async def close_session(self):
        if self.session:
            await self.session.close()
            self.session = None

    async def handle_mcp_jsonrpc(self, request, data):
        """Handle MCP JSON-RPC format requests"""
        method = data.get("method", "")
        request_id = data.get("id", 1)
        
        logger.info(f"Received MCP JSON-RPC method: {method}")
        
        if method == "tools/list":
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
            
            return web.json_response({
                "jsonrpc": "2.0",
                "id": request_id,
                "result": {
                    "tools": tools
                }
            })
        
        elif method == "tools/call":
            # Handle tool calls
            params = data.get("params", {})
            tool_name = params.get("name", "")
            arguments = params.get("arguments", {})
            
            logger.info(f"Tool call: {tool_name} with args: {arguments}")
            
            # For now, return a simple response
            return web.json_response({
                "jsonrpc": "2.0",
                "id": request_id,
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": f"Tool {tool_name} called with arguments: {arguments}"
                        }
                    ]
                }
            })
        
        else:
            return web.json_response({
                "jsonrpc": "2.0",
                "id": request_id,
                "error": {
                    "code": -32601,
                    "message": f"Method not found: {method}"
                }
            })

    async def handle_command(self, request):
        """Handle both StreamableHTTP and MCP JSON-RPC commands"""
        try:
            data = await request.json()
            
            # Check if it's MCP JSON-RPC format
            if "jsonrpc" in data and "method" in data:
                return await self.handle_mcp_jsonrpc(request, data)
            
            # Handle StreamableHTTP format
            command = data.get("command", "")
            request_id = data.get("request_id", "")
            payload = data.get("payload", {})
            
            logger.info(f"Received StreamableHTTP command: {command}")
            
            # Handle MCP discovery commands
            if command == "mcp.list_tools" or command == "tools.list":
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
                
                return web.json_response({
                    "success": True,
                    "request_id": request_id,
                    "payload": {
                        "tools": tools,
                        "count": len(tools)
                    }
                })
            
            # Handle regular KV commands by forwarding to real Synap
            elif command.startswith("kv."):
                await self.init_session()
                try:
                    async with self.session.post(
                        f"{SYNAP_BASE_URL}/api/v1/command",
                        json=data,
                        headers={"Content-Type": "application/json"}
                    ) as response:
                        if response.status == 200:
                            result = await response.json()
                            return web.json_response(result)
                        else:
                            error_text = await response.text()
                            return web.json_response({
                                "success": False,
                                "request_id": request_id,
                                "error": f"Synap error {response.status}: {error_text}"
                            })
                except Exception as e:
                    return web.json_response({
                        "success": False,
                        "request_id": request_id,
                        "error": f"Connection error: {str(e)}"
                    })
            
            # Unknown command
            else:
                return web.json_response({
                    "success": False,
                    "request_id": request_id,
                    "error": f"Unknown command: {command}"
                })
                
        except Exception as e:
            logger.error(f"Error handling command: {e}")
            return web.json_response({
                "success": False,
                "request_id": "error",
                "error": f"Internal error: {str(e)}"
            }, status=500)

    async def handle_health(self, request):
        """Health check endpoint"""
        return web.json_response({
            "service": "synap-bridge",
            "status": "healthy",
            "version": "0.1.0"
        })
    
    async def handle_root(self, request):
        """Root endpoint - returns MCP service information"""
        if request.method == "GET":
            return web.json_response({
                "service": "synap-bridge",
                "version": "0.1.0",
                "protocol": "MCP HTTP",
                "endpoints": {
                    "/mcp": "MCP JSON-RPC endpoint",
                    "/api/v1/command": "StreamableHTTP endpoint",
                    "/health": "Health check endpoint"
                },
                "status": "running",
                "tools": [
                    "synap_kv_get",
                    "synap_kv_set",
                    "synap_queue_publish"
                ]
            })
        else:
            # For POST requests to root, try to handle as MCP
            try:
                data = await request.json()
                if "jsonrpc" in data and "method" in data:
                    return await self.handle_mcp_jsonrpc(request, data)
                else:
                    return await self.handle_command(request)
            except:
                return web.json_response({
                    "error": "Invalid request",
                    "message": "Use /mcp for MCP JSON-RPC or /api/v1/command for StreamableHTTP"
                }, status=400)

async def create_app():
    """Create the web application"""
    bridge = SynapHTTPBridge()
    
    app = web.Application()
    
    # Add CORS headers
    @web.middleware
    async def add_cors(request, handler):
        response = await handler(request)
        response.headers['Access-Control-Allow-Origin'] = '*'
        response.headers['Access-Control-Allow-Methods'] = 'GET, POST, OPTIONS'
        response.headers['Access-Control-Allow-Headers'] = 'Content-Type, Authorization'
        return response
    
    app.middlewares.append(add_cors)
    
    # Routes
    app.router.add_get('/', bridge.handle_root)  # Root endpoint
    app.router.add_post('/', bridge.handle_root)  # Root POST for MCP
    app.router.add_post('/api/v1/command', bridge.handle_command)
    app.router.add_post('/mcp', bridge.handle_command)  # MCP endpoint for Cursor
    app.router.add_get('/health', bridge.handle_health)
    
    # Cleanup on shutdown
    async def cleanup_context(app):
        yield
        await bridge.close_session()
    
    app.cleanup_ctx.append(cleanup_context)
    
    return app

async def main():
    """Run the bridge server"""
    app = await create_app()
    
    runner = web.AppRunner(app)
    await runner.setup()
    
    site = web.TCPSite(runner, '127.0.0.1', 15500)
    await site.start()
    
    logger.info("Synap HTTP Bridge running on http://127.0.0.1:15500")
    logger.info("- / - Root endpoint (GET/POST)")
    logger.info("- /mcp - MCP JSON-RPC endpoint")
    logger.info("- /api/v1/command - StreamableHTTP commands")
    logger.info("- /health - Health check")
    
    try:
        await asyncio.Future()  # Run forever
    except KeyboardInterrupt:
        logger.info("Shutting down...")
    finally:
        await runner.cleanup()

if __name__ == "__main__":
    asyncio.run(main())
