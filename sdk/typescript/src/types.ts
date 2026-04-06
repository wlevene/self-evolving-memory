/**
 * Memory types and models for TypeScript SDK
 */

/**
 * Memory pool type - determines storage and retrieval strategy
 */
export enum MemoryPool {
  Explicit = "explicit",
  Implicit = "implicit",
}

/**
 * Memory type classification
 */
export enum MemoryType {
  Fact = "fact",
  Event = "event",
  Procedure = "procedure",
  Concept = "concept",
  Preference = "preference",
  Context = "context",
}

/**
 * Memory relationship types
 */
export enum LinkType {
  Causes = "causes",
  Related = "related",
  Contradicts = "contradicts",
  Specializes = "specializes",
  DerivedFrom = "derived_from",
  Similar = "similar",
  Follows = "follows",
  Alternative = "alternative",
}

/**
 * Core memory structure
 */
export interface Memory {
  id: string;
  content: string;
  pool: MemoryPool;
  type: MemoryType;
  confidence: number;
  importance: number;
  decay_rate: number;
  created_at: string;
  last_accessed: string;
  access_count: number;
  tags: string[];
  source?: string;
  embedding?: number[];
  metadata: Record<string, unknown>;
}

/**
 * Link between memories
 */
export interface MemoryLink {
  id: string;
  source_id: string;
  target_id: string;
  link_type: LinkType;
  strength: number;
  confidence: number;
  created_at: string;
  context?: string;
  metadata: Record<string, unknown>;
}

/**
 * Request to create a new memory
 */
export interface CreateMemoryRequest {
  content: string;
  pool?: MemoryPool;
  type?: MemoryType;
  confidence?: number;
  importance?: number;
  decay_rate?: number;
  tags?: string[];
  source?: string;
  metadata?: Record<string, unknown>;
}

/**
 * Request to update an existing memory
 */
export interface UpdateMemoryRequest {
  content?: string;
  pool?: MemoryPool;
  type?: MemoryType;
  confidence?: number;
  importance?: number;
  decay_rate?: number;
  tags?: string[];
  metadata?: Record<string, unknown>;
}

/**
 * Memory search query
 */
export interface SearchQuery {
  query: string;
  pool?: MemoryPool;
  type?: MemoryType;
  tags?: string[];
  limit?: number;
  min_confidence?: number;
  include_links?: boolean;
}

/**
 * Request to create a link
 */
export interface CreateLinkRequest {
  source_id: string;
  target_id: string;
  link_type: LinkType;
  strength?: number;
  context?: string;
  metadata?: Record<string, unknown>;
}

/**
 * Statistics about the memory system
 */
export interface MemoryStats {
  total_memories: number;
  explicit_count: number;
  implicit_count: number;
  total_links: number;
  by_type: Record<string, number>;
  by_tag: Record<string, number>;
  avg_confidence: number;
  avg_importance: number;
}

/**
 * API response wrapper
 */
export interface ApiResponse<T> {
  status: number;
  data: T;
  error?: string;
}

/**
 * Search response
 */
export interface SearchResponse {
  results: Memory[];
  count: number;
  query: string;
}

/**
 * Links response
 */
export interface LinksResponse {
  links: MemoryLink[];
  count: number;
}

/**
 * Delete response
 */
export interface DeleteResponse {
  deleted: boolean;
  id?: string;
}

/**
 * Health check response
 */
export interface HealthResponse {
  status: string;
  service: string;
}