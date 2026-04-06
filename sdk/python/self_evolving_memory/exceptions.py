"""
Custom exceptions for Python SDK
"""


class MemoryError(Exception):
    """Base exception for memory operations"""
    pass


class MemoryNotFoundError(MemoryError):
    """Memory not found"""
    pass


class ValidationError(MemoryError):
    """Validation error"""
    pass


class ConnectionError(MemoryError):
    """Connection error"""
    pass


class AuthenticationError(MemoryError):
    """Authentication error"""
    pass


class RateLimitError(MemoryError):
    """Rate limit exceeded"""
    pass