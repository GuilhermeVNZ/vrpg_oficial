#!/usr/bin/env python3
"""
Synap MCP Bridge Server
Exposes Synap HTTP API as MCP tools for Cursor integration
"""

import asyncio
import json
import sys
from typing import Any, Dict, List, Optional
import aiohttp
from mcp.server import Server
from mcp.server.models import InitializationOptions
import mcp.server.stdio
import mcp.types as types

# Synap server configuration
SYNAP_BASE_URL = "http://127.0.0.1:15500"

class SynapMCPServer:
    def __init__(self):
        self.server = Server("synap")
        self.session: Optional[aiohttp.ClientSession] = None
        
    async def init_session(self):
        """Initialize HTTP session for Synap communication"""
        if not self.session:
            self.session = aiohttp.ClientSession()
    
    async def close_session(self):
        """Close HTTP session"""
        if self.session:
            await self.session.close()
            self.session = None
    
    async def call_synap_mcp(self, tool_name: str, arguments: Dict[str, Any]) -> Dict[str, Any]:
        """Call Synap MCP endpoint via HTTP"""
        await self.init_session()
        
        payload = {
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments
            }
        }
        
        try:
            async with self.session.post(
                f"{SYNAP_BASE_URL}/mcp",
                json=payload,
                headers={"Content-Type": "application/json"}
            ) as response:
                if response.status == 200:
                    result = await response.json()
                    return result.get("result", {})
                else:
                    error_text = await response.text()
                    raise Exception(f"Synap HTTP error {response.status}: {error_text}")
        except Exception as e:
            raise Exception(f"Failed to call Synap: {str(e)}")

# Create server instance
synap_server = SynapMCPServer()

@synap_server.server.list_tools()
async def handle_list_tools() -> List[types.Tool]:
    """List available Synap MCP tools"""
    return [
        types.Tool(
            name="synap_kv_get",
            description="Retrieve a value from Synap key-value store",
            inputSchema={
                "type": "object",
                "properties": {
                    "key": {"type": "string", "description": "The key to retrieve"}
                },
                "required": ["key"]
            }
        ),
        types.Tool(
            name="synap_kv_set", 
            description="Store a value in Synap key-value store",
            inputSchema={
                "type": "object",
                "properties": {
                    "key": {"type": "string", "description": "The key to store"},
                    "value": {"type": "string", "description": "The value to store"},
                    "ttl": {"type": "integer", "description": "Time to live in seconds (optional)"}
                },
                "required": ["key", "value"]
            }
        ),
        types.Tool(
            name="synap_kv_delete",
            description="Delete a key from Synap key-value store", 
            inputSchema={
                "type": "object",
                "properties": {
                    "key": {"type": "string", "description": "The key to delete"}
                },
                "required": ["key"]
            }
        ),
        types.Tool(
            name="synap_kv_scan",
            description="Scan keys by prefix in Synap key-value store",
            inputSchema={
                "type": "object", 
                "properties": {
                    "prefix": {"type": "string", "description": "Key prefix to scan (optional)"},
                    "limit": {"type": "integer", "description": "Maximum number of keys to return (optional)"}
                },
                "required": []
            }
        ),
        types.Tool(
            name="synap_queue_publish",
            description="Publish a message to Synap queue",
            inputSchema={
                "type": "object",
                "properties": {
                    "queue": {"type": "string", "description": "Queue name"},
                    "message": {"type": "string", "description": "Message content"},
                    "priority": {"type": "integer", "description": "Message priority 0-9 (optional)"}
                },
                "required": ["queue", "message"]
            }
        ),
        types.Tool(
            name="synap_queue_consume",
            description="Consume a message from Synap queue",
            inputSchema={
                "type": "object",
                "properties": {
                    "queue": {"type": "string", "description": "Queue name"},
                    "consumer_id": {"type": "string", "description": "Consumer identifier"}
                },
                "required": ["queue", "consumer_id"]
            }
        ),
        types.Tool(
            name="synap_stream_publish",
            description="Publish an event to Synap stream",
            inputSchema={
                "type": "object",
                "properties": {
                    "room": {"type": "string", "description": "Stream room name"},
                    "event": {"type": "string", "description": "Event type"},
                    "data": {"type": "object", "description": "Event data"}
                },
                "required": ["room", "event", "data"]
            }
        ),
        types.Tool(
            name="synap_pubsub_publish",
            description="Publish a message to Synap pub/sub topic",
            inputSchema={
                "type": "object",
                "properties": {
                    "topic": {"type": "string", "description": "Topic name"},
                    "message": {"type": "object", "description": "Message data"}
                },
                "required": ["topic", "message"]
            }
        )
    ]

@synap_server.server.call_tool()
async def handle_call_tool(name: str, arguments: Dict[str, Any]) -> List[types.TextContent]:
    """Handle tool calls by forwarding to Synap"""
    try:
        result = await synap_server.call_synap_mcp(name, arguments)
        return [types.TextContent(type="text", text=json.dumps(result, indent=2))]
    except Exception as e:
        error_msg = f"Error calling {name}: {str(e)}"
        return [types.TextContent(type="text", text=error_msg)]

async def main():
    """Run the MCP server"""
    try:
        async with mcp.server.stdio.stdio_server() as (read_stream, write_stream):
            await synap_server.server.run(
                read_stream,
                write_stream,
                InitializationOptions(
                    server_name="synap",
                    server_version="0.1.0",
                    capabilities=synap_server.server.get_capabilities(
                        notification_options=None,
                        experimental_capabilities={}
                    )
                )
            )
    finally:
        await synap_server.close_session()

if __name__ == "__main__":
    asyncio.run(main())
