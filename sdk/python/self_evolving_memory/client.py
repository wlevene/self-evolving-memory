"""
Memory client for Python SDK - sync and async implementations
"""

import httpx
from typing import Optional, List, Dict, Any
from .models import (
    Memory, MemoryPool, MemoryType, MemoryLink, LinkType,
    CreateMemoryRequest, UpdateMemoryRequest, SearchQuery, MemoryStats
)
from .exceptions import MemoryNotFoundError, ValidationError, ConnectionError


class BaseClient:
    """Base client with shared logic"""
    
    def __init__(self, base_url: str = "http://localhost:3000"):
        self.base_url = base_url.rstrip("/")
    
    def _handle_response(self, response: httpx.Response) -> Dict[str, Any]:
        """Handle HTTP response and raise exceptions if needed"""
        if response.status_code == 404:
            raise MemoryNotFoundError("Memory not found")
        if response.status_code == 400:
            data = response.json()
            raise ValidationError(data.get("error", "Validation error"))
        if response.status_code >= 500:
            raise ConnectionError(f"Server error: {response.status_code}")
        
        return response.json()
    
    def _get_endpoint(self, path: str) -> str:
        """Build full endpoint URL"""
        return f"{self.base_url}{path}"


class MemoryClient(BaseClient):
    """Synchronous memory client"""
    
    def __init__(self, base_url: str = "http://localhost:3000"):
        super().__init__(base_url)
        self._client = httpx.Client(timeout=30.0)
    
    def close(self) -> None:
        """Close the client"""
        self._client.close()
    
    def __enter__(self) -> "MemoryClient":
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb) -> None:
        self.close()
    
    def create(self, request: CreateMemoryRequest) -> Memory:
        """Create a new memory"""
        response = self._client.post(
            self._get_endpoint("/memories"),
            json=request.to_dict()
        )
        data = self._handle_response(response)
        return Memory.from_dict(data)
    
    def get(self, memory_id: str, include_links: bool = False) -> Memory:
        """Get a memory by ID"""
        params = {"include_links": include_links} if include_links else {}
        response = self._client.get(
            self._get_endpoint(f"/memories/{memory_id}"),
            params=params
        )
        data = self._handle_response(response)
        return Memory.from_dict(data)
    
    def update(self, memory_id: str, request: UpdateMemoryRequest) -> Memory:
        """Update an existing memory"""
        response = self._client.put(
            self._get_endpoint(f"/memories/{memory_id}"),
            json=request.to_dict()
        )
        data = self._handle_response(response)
        return Memory.from_dict(data)
    
    def delete(self, memory_id: str) -> bool:
        """Delete a memory"""
        response = self._client.delete(
            self._get_endpoint(f"/memories/{memory_id}")
        )
        data = self._handle_response(response)
        return data.get("deleted", False)
    
    def search(self, query: SearchQuery) -> List[Memory]:
        """Search memories"""
        response = self._client.get(
            self._get_endpoint("/memories/search"),
            params=query.to_dict()
        )
        data = self._handle_response(response)
        return [Memory.from_dict(m) for m in data.get("results", [])]
    
    def list(self, pool: Optional[MemoryPool] = None, limit: int = 50) -> List[Memory]:
        """List memories"""
        params = {"limit": limit}
        if pool is not None:
            params["pool"] = pool.value
        
        response = self._client.get(
            self._get_endpoint("/memories"),
            params=params
        )
        data = self._handle_response(response)
        return [Memory.from_dict(m) for m in data.get("results", [])]
    
    def create_link(
        self,
        source_id: str,
        target_id: str,
        link_type: LinkType,
        strength: float = 0.8,
        context: Optional[str] = None
    ) -> MemoryLink:
        """Create a link between memories"""
        response = self._client.post(
            self._get_endpoint("/links"),
            json={
                "source_id": source_id,
                "target_id": target_id,
                "link_type": link_type.value,
                "strength": strength,
                "context": context,
            }
        )
        data = self._handle_response(response)
        return MemoryLink.from_dict(data)
    
    def get_links(self, memory_id: str) -> List[MemoryLink]:
        """Get all links for a memory"""
        response = self._client.get(
            self._get_endpoint(f"/memories/{memory_id}/links")
        )
        data = self._handle_response(response)
        return [MemoryLink.from_dict(l) for l in data.get("links", [])]
    
    def delete_link(self, link_id: str) -> bool:
        """Delete a link"""
        response = self._client.delete(
            self._get_endpoint(f"/links/{link_id}")
        )
        data = self._handle_response(response)
        return data.get("deleted", False)
    
    def stats(self) -> MemoryStats:
        """Get system statistics"""
        response = self._client.get(self._get_endpoint("/stats"))
        data = self._handle_response(response)
        return MemoryStats.from_dict(data)
    
    def health_check(self) -> Dict[str, Any]:
        """Check system health"""
        response = self._client.get(self._get_endpoint("/health"))
        return self._handle_response(response)


class AsyncMemoryClient(BaseClient):
    """Asynchronous memory client"""
    
    def __init__(self, base_url: str = "http://localhost:3000"):
        super().__init__(base_url)
        self._client = httpx.AsyncClient(timeout=30.0)
    
    async def close(self) -> None:
        """Close the client"""
        await self._client.aclose()
    
    async def __aenter__(self) -> "AsyncMemoryClient":
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb) -> None:
        await self.close()
    
    async def create(self, request: CreateMemoryRequest) -> Memory:
        """Create a new memory"""
        response = await self._client.post(
            self._get_endpoint("/memories"),
            json=request.to_dict()
        )
        data = self._handle_response(response)
        return Memory.from_dict(data)
    
    async def get(self, memory_id: str, include_links: bool = False) -> Memory:
        """Get a memory by ID"""
        params = {"include_links": include_links} if include_links else {}
        response = await self._client.get(
            self._get_endpoint(f"/memories/{memory_id}"),
            params=params
        )
        data = self._handle_response(response)
        return Memory.from_dict(data)
    
    async def update(self, memory_id: str, request: UpdateMemoryRequest) -> Memory:
        """Update an existing memory"""
        response = await self._client.put(
            self._get_endpoint(f"/memories/{memory_id}"),
            json=request.to_dict()
        )
        data = self._handle_response(response)
        return Memory.from_dict(data)
    
    async def delete(self, memory_id: str) -> bool:
        """Delete a memory"""
        response = await self._client.delete(
            self._get_endpoint(f"/memories/{memory_id}")
        )
        data = self._handle_response(response)
        return data.get("deleted", False)
    
    async def search(self, query: SearchQuery) -> List[Memory]:
        """Search memories"""
        response = await self._client.get(
            self._get_endpoint("/memories/search"),
            params=query.to_dict()
        )
        data = self._handle_response(response)
        return [Memory.from_dict(m) for m in data.get("results", [])]
    
    async def list(self, pool: Optional[MemoryPool] = None, limit: int = 50) -> List[Memory]:
        """List memories"""
        params = {"limit": limit}
        if pool is not None:
            params["pool"] = pool.value
        
        response = await self._client.get(
            self._get_endpoint("/memories"),
            params=params
        )
        data = self._handle_response(response)
        return [Memory.from_dict(m) for m in data.get("results", [])]
    
    async def create_link(
        self,
        source_id: str,
        target_id: str,
        link_type: LinkType,
        strength: float = 0.8,
        context: Optional[str] = None
    ) -> MemoryLink:
        """Create a link between memories"""
        response = await self._client.post(
            self._get_endpoint("/links"),
            json={
                "source_id": source_id,
                "target_id": target_id,
                "link_type": link_type.value,
                "strength": strength,
                "context": context,
            }
        )
        data = self._handle_response(response)
        return MemoryLink.from_dict(data)
    
    async def get_links(self, memory_id: str) -> List[MemoryLink]:
        """Get all links for a memory"""
        response = await self._client.get(
            self._get_endpoint(f"/memories/{memory_id}/links")
        )
        data = self._handle_response(response)
        return [MemoryLink.from_dict(l) for l in data.get("links", [])]
    
    async def delete_link(self, link_id: str) -> bool:
        """Delete a link"""
        response = await self._client.delete(
            self._get_endpoint(f"/links/{link_id}")
        )
        data = self._handle_response(response)
        return data.get("deleted", False)
    
    async def stats(self) -> MemoryStats:
        """Get system statistics"""
        response = await self._client.get(self._get_endpoint("/stats"))
        data = self._handle_response(response)
        return MemoryStats.from_dict(data)
    
    async def health_check(self) -> Dict[str, Any]:
        """Check system health"""
        response = await self._client.get(self._get_endpoint("/health"))
        return self._handle_response(response)