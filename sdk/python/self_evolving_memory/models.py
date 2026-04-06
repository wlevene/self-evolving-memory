"""
Memory models and types for Python SDK
"""

from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import List, Optional, Dict, Any
import uuid


class MemoryPool(Enum):
    """Memory pool type - determines storage and retrieval strategy"""
    EXPLICIT = "explicit"
    IMPLICIT = "implicit"


class MemoryType(Enum):
    """Memory type classification"""
    FACT = "fact"
    EVENT = "event"
    PROCEDURE = "procedure"
    CONCEPT = "concept"
    PREFERENCE = "preference"
    CONTEXT = "context"


class LinkType(Enum):
    """Memory relationship types"""
    CAUSES = "causes"
    RELATED = "related"
    CONTRADICTS = "contradicts"
    SPECIALIZES = "specializes"
    DERIVED_FROM = "derived_from"
    SIMILAR = "similar"
    FOLLOWS = "follows"
    ALTERNATIVE = "alternative"


@dataclass
class Memory:
    """Core memory structure"""
    id: str
    content: str
    pool: MemoryPool
    type: MemoryType
    confidence: float = 0.8
    importance: float = 0.5
    decay_rate: float = 0.01
    created_at: datetime = None
    last_accessed: datetime = None
    access_count: int = 0
    tags: List[str] = field(default_factory=list)
    source: Optional[str] = None
    embedding: Optional[List[float]] = None
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def __post_init__(self):
        if self.created_at is None:
            self.created_at = datetime.utcnow()
        if self.last_accessed is None:
            self.last_accessed = datetime.utcnow()
        if not self.id:
            self.id = str(uuid.uuid4())
    
    def current_strength(self) -> float:
        """Calculate current strength based on decay and access"""
        if self.last_accessed is None:
            return self.importance
        
        elapsed_hours = (datetime.utcnow() - self.last_accessed).total_seconds() / 3600
        decay_factor = 1.0 - (self.decay_rate * elapsed_hours / 24.0)
        access_boost = 1.0 + (self.access_count * 0.1)
        return min(max(self.importance * decay_factor * access_boost, 0.0), 1.0)
    
    def record_access(self) -> None:
        """Record an access event"""
        self.last_accessed = datetime.utcnow()
        self.access_count += 1
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for API calls"""
        return {
            "id": self.id,
            "content": self.content,
            "pool": self.pool.value,
            "type": self.type.value,
            "confidence": self.confidence,
            "importance": self.importance,
            "decay_rate": self.decay_rate,
            "created_at": self.created_at.isoformat() if self.created_at else None,
            "last_accessed": self.last_accessed.isoformat() if self.last_accessed else None,
            "access_count": self.access_count,
            "tags": self.tags,
            "source": self.source,
            "metadata": self.metadata,
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Memory":
        """Create Memory from dictionary (API response)"""
        return cls(
            id=data["id"],
            content=data["content"],
            pool=MemoryPool(data["pool"]),
            type=MemoryType(data.get("type", "fact")),
            confidence=data.get("confidence", 0.8),
            importance=data.get("importance", 0.5),
            decay_rate=data.get("decay_rate", 0.01),
            created_at=datetime.fromisoformat(data["created_at"]) if data.get("created_at") else None,
            last_accessed=datetime.fromisoformat(data["last_accessed"]) if data.get("last_accessed") else None,
            access_count=data.get("access_count", 0),
            tags=data.get("tags", []),
            source=data.get("source"),
            embedding=data.get("embedding"),
            metadata=data.get("metadata", {}),
        )


@dataclass
class MemoryLink:
    """Link between memories"""
    id: str
    source_id: str
    target_id: str
    link_type: LinkType
    strength: float = 0.8
    confidence: float = 0.8
    created_at: datetime = None
    context: Optional[str] = None
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def __post_init__(self):
        if self.created_at is None:
            self.created_at = datetime.utcnow()
        if not self.id:
            self.id = str(uuid.uuid4())
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for API calls"""
        return {
            "id": self.id,
            "source_id": self.source_id,
            "target_id": self.target_id,
            "link_type": self.link_type.value,
            "strength": self.strength,
            "confidence": self.confidence,
            "created_at": self.created_at.isoformat() if self.created_at else None,
            "context": self.context,
            "metadata": self.metadata,
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "MemoryLink":
        """Create MemoryLink from dictionary (API response)"""
        return cls(
            id=data["id"],
            source_id=data["source_id"],
            target_id=data["target_id"],
            link_type=LinkType(data["link_type"]),
            strength=data.get("strength", 0.8),
            confidence=data.get("confidence", 0.8),
            created_at=datetime.fromisoformat(data["created_at"]) if data.get("created_at") else None,
            context=data.get("context"),
            metadata=data.get("metadata", {}),
        )


@dataclass
class CreateMemoryRequest:
    """Request to create a new memory"""
    content: str
    pool: MemoryPool = MemoryPool.EXPLICIT
    type: MemoryType = MemoryType.FACT
    confidence: float = 0.8
    importance: float = 0.5
    decay_rate: float = 0.01
    tags: List[str] = field(default_factory=list)
    source: Optional[str] = None
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            "content": self.content,
            "pool": self.pool.value,
            "type": self.type.value,
            "confidence": self.confidence,
            "importance": self.importance,
            "decay_rate": self.decay_rate,
            "tags": self.tags,
            "source": self.source,
            "metadata": self.metadata,
        }


@dataclass
class UpdateMemoryRequest:
    """Request to update an existing memory"""
    content: Optional[str] = None
    pool: Optional[MemoryPool] = None
    type: Optional[MemoryType] = None
    confidence: Optional[float] = None
    importance: Optional[float] = None
    decay_rate: Optional[float] = None
    tags: Optional[List[str]] = None
    metadata: Optional[Dict[str, Any]] = None
    
    def to_dict(self) -> Dict[str, Any]:
        result = {}
        if self.content is not None:
            result["content"] = self.content
        if self.pool is not None:
            result["pool"] = self.pool.value
        if self.type is not None:
            result["type"] = self.type.value
        if self.confidence is not None:
            result["confidence"] = self.confidence
        if self.importance is not None:
            result["importance"] = self.importance
        if self.decay_rate is not None:
            result["decay_rate"] = self.decay_rate
        if self.tags is not None:
            result["tags"] = self.tags
        if self.metadata is not None:
            result["metadata"] = self.metadata
        return result


@dataclass
class SearchQuery:
    """Memory search query"""
    query: str
    pool: Optional[MemoryPool] = None
    type: Optional[MemoryType] = None
    tags: Optional[List[str]] = None
    limit: int = 10
    min_confidence: float = 0.5
    include_links: bool = False
    
    def to_dict(self) -> Dict[str, Any]:
        result = {
            "query": self.query,
            "limit": self.limit,
            "min_confidence": self.min_confidence,
            "include_links": self.include_links,
        }
        if self.pool is not None:
            result["pool"] = self.pool.value
        if self.type is not None:
            result["type"] = self.type.value
        if self.tags is not None:
            result["tags"] = self.tags
        return result


@dataclass
class MemoryStats:
    """Statistics about the memory system"""
    total_memories: int
    explicit_count: int
    implicit_count: int
    total_links: int
    by_type: Dict[str, int]
    by_tag: Dict[str, int]
    avg_confidence: float
    avg_importance: float
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "MemoryStats":
        return cls(
            total_memories=data.get("total_memories", 0),
            explicit_count=data.get("explicit_count", 0),
            implicit_count=data.get("implicit_count", 0),
            total_links=data.get("total_links", 0),
            by_type=data.get("by_type", {}),
            by_tag=data.get("by_tag", {}),
            avg_confidence=data.get("avg_confidence", 0.0),
            avg_importance=data.get("avg_importance", 0.0),
        )