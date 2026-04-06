/**
 * Memory client for TypeScript SDK
 */

import {
  Memory,
  MemoryLink,
  CreateMemoryRequest,
  UpdateMemoryRequest,
  SearchQuery,
  CreateLinkRequest,
  MemoryStats,
  SearchResponse,
  LinksResponse,
  DeleteResponse,
  HealthResponse,
  MemoryPool,
  MemoryType,
  LinkType,
} from "./types";

/**
 * Memory client class
 */
export class MemoryClient {
  private baseUrl: string;

  constructor(baseUrl: string = "http://localhost:3000") {
    this.baseUrl = baseUrl.replace(/\/$/, "");
  }

  private async request<T>(
    path: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${path}`;
    const response = await fetch(url, {
      ...options,
      headers: {
        "Content-Type": "application/json",
        ...options.headers,
      },
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ error: "Unknown error" }));
      throw new Error(error.error || `HTTP ${response.status}`);
    }

    return response.json();
  }

  /**
   * Create a new memory
   */
  async create(request: CreateMemoryRequest): Promise<Memory> {
    return this.request<Memory>("/memories", {
      method: "POST",
      body: JSON.stringify(request),
    });
  }

  /**
   * Get a memory by ID
   */
  async get(id: string, includeLinks = false): Promise<Memory> {
    const params = includeLinks ? "?include_links=true" : "";
    return this.request<Memory>(`/memories/${id}${params}`);
  }

  /**
   * Update an existing memory
   */
  async update(id: string, request: UpdateMemoryRequest): Promise<Memory> {
    return this.request<Memory>(`/memories/${id}`, {
      method: "PUT",
      body: JSON.stringify(request),
    });
  }

  /**
   * Delete a memory
   */
  async delete(id: string): Promise<boolean> {
    const result = await this.request<DeleteResponse>(`/memories/${id}`, {
      method: "DELETE",
    });
    return result.deleted;
  }

  /**
   * Search memories
   */
  async search(query: SearchQuery): Promise<Memory[]> {
    const params = new URLSearchParams();
    params.set("query", query.query);
    if (query.pool) params.set("pool", query.pool);
    if (query.type) params.set("type", query.type);
    if (query.limit) params.set("limit", String(query.limit));
    if (query.min_confidence) params.set("min_confidence", String(query.min_confidence));
    if (query.tags) params.set("tags", query.tags.join(","));

    const result = await this.request<SearchResponse>(`/memories/search?${params}`);
    return result.results;
  }

  /**
   * List memories
   */
  async list(pool?: MemoryPool, limit = 50): Promise<Memory[]> {
    const params = new URLSearchParams();
    params.set("limit", String(limit));
    if (pool) params.set("pool", pool);

    const result = await this.request<{ results: Memory[]; count: number }>(
      `/memories?${params}`
    );
    return result.results;
  }

  /**
   * Create a link between memories
   */
  async createLink(request: CreateLinkRequest): Promise<MemoryLink> {
    return this.request<MemoryLink>("/links", {
      method: "POST",
      body: JSON.stringify(request),
    });
  }

  /**
   * Get all links for a memory
   */
  async getLinks(memoryId: string): Promise<MemoryLink[]> {
    const result = await this.request<LinksResponse>(`/memories/${memoryId}/links`);
    return result.links;
  }

  /**
   * Delete a link
   */
  async deleteLink(linkId: string): Promise<boolean> {
    const result = await this.request<DeleteResponse>(`/links/${linkId}`, {
      method: "DELETE",
    });
    return result.deleted;
  }

  /**
   * Get system statistics
   */
  async stats(): Promise<MemoryStats> {
    return this.request<MemoryStats>("/stats");
  }

  /**
   * Check system health
   */
  async health(): Promise<HealthResponse> {
    return this.request<HealthResponse>("/health");
  }

  /**
   * Create memory in explicit pool (convenience method)
   */
  async createExplicit(content: string, type: MemoryType = MemoryType.Fact): Promise<Memory> {
    return this.create({ content, pool: MemoryPool.Explicit, type });
  }

  /**
   * Create memory in implicit pool (convenience method)
   */
  async createImplicit(content: string, type: MemoryType = MemoryType.Preference): Promise<Memory> {
    return this.create({ content, pool: MemoryPool.Implicit, type });
  }

  /**
   * Link two memories (convenience method)
   */
  async link(
    sourceId: string,
    targetId: string,
    linkType: LinkType = LinkType.Related,
    strength = 0.8
  ): Promise<MemoryLink> {
    return this.createLink({
      source_id: sourceId,
      target_id: targetId,
      link_type: linkType,
      strength,
    });
  }
}

// Export types
export * from "./types";