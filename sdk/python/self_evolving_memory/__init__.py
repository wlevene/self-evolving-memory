"""
Self-Evolving Memory Python SDK

A Python SDK for interacting with the Self-Evolving Memory System.
Provides both sync and async APIs for memory operations.
"""

__version__ = "0.1.0"
__author__ = "OpenClaw Team"

from .client import MemoryClient, AsyncMemoryClient
from .models import Memory, MemoryPool, MemoryType, LinkType, MemoryLink
from .exceptions import MemoryNotFoundError, ValidationError, ConnectionError

__all__ = [
    "MemoryClient",
    "AsyncMemoryClient",
    "Memory",
    "MemoryPool",
    "MemoryType",
    "LinkType",
    "MemoryLink",
    "MemoryNotFoundError",
    "ValidationError",
    "ConnectionError",
]