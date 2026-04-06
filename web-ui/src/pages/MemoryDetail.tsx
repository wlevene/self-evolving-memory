import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { useParams, Link, useNavigate } from 'react-router-dom'
import { api } from '../api/client'
import { formatDistanceToNow, format } from 'date-fns'
import { 
  Brain, 
  Link2, 
  Eye, 
  Trash2, 
  Edit, 
  ArrowLeft,
  Tag,
  Calendar,
  Target,
  TrendingUp
} from 'lucide-react'

export function MemoryDetail() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const queryClient = useQueryClient()

  const { data: memory, isLoading } = useQuery({
    queryKey: ['memory', id],
    queryFn: async () => {
      const response = await api.get(`/memories/${id}?include_links=true`)
      return response.data.memory || response.data
    },
    enabled: !!id,
  })

  const { data: links } = useQuery({
    queryKey: ['links', id],
    queryFn: async () => {
      const response = await api.get(`/memories/${id}/links`)
      return response.data.links
    },
    enabled: !!id,
  })

  const deleteMutation = useMutation({
    mutationFn: async () => {
      await api.delete(`/memories/${id}`)
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['memories'] })
      navigate('/memories')
    },
  })

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
      </div>
    )
  }

  if (!memory) {
    return (
      <div className="text-center py-12">
        <Brain className="w-12 h-12 text-gray-400 mx-auto mb-4" />
        <p className="text-gray-500">Memory not found</p>
        <Link to="/memories" className="mt-4 text-indigo-600 hover:text-indigo-800">
          Back to list
        </Link>
      </div>
    )
  }

  const strength = memory.current_strength || 
    (memory.importance * (1 + memory.access_count * 0.1))

  return (
    <div className="space-y-6">
      {/* Back button */}
      <Link
        to="/memories"
        className="inline-flex items-center gap-1 text-sm text-gray-600 hover:text-gray-900"
      >
        <ArrowLeft className="w-4 h-4" />
        Back to memories
      </Link>

      {/* Main content */}
      <div className="bg-white rounded-lg shadow overflow-hidden">
        {/* Header */}
        <div className="bg-gray-50 px-6 py-4 border-b border-gray-200">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <Brain className="w-6 h-6 text-indigo-600" />
              <span className={`px-3 py-1 text-sm font-medium rounded-full ${
                memory.pool === 'explicit'
                  ? 'bg-green-100 text-green-800'
                  : 'bg-purple-100 text-purple-800'
              }`}>
                {memory.pool}
              </span>
              <span className="px-3 py-1 text-sm font-medium rounded-full bg-gray-100 text-gray-800">
                {memory.type}
              </span>
            </div>
            <div className="flex items-center gap-2">
              <button
                onClick={() => deleteMutation.mutate()}
                className="text-red-600 hover:text-red-800 p-2"
                disabled={deleteMutation.isPending}
              >
                <Trash2 className="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>

        {/* Content */}
        <div className="px-6 py-6">
          <div className="prose max-w-none">
            <p className="text-gray-900 whitespace-pre-wrap">{memory.content}</p>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 px-6 py-4 bg-gray-50 border-t border-gray-200">
          <div>
            <div className="flex items-center gap-1 text-sm text-gray-500 mb-1">
              <Target className="w-4 h-4" />
              Confidence
            </div>
            <div className="text-lg font-bold text-gray-900">
              {(memory.confidence * 100).toFixed(0)}%
            </div>
          </div>
          <div>
            <div className="flex items-center gap-1 text-sm text-gray-500 mb-1">
              <TrendingUp className="w-4 h-4" />
              Importance
            </div>
            <div className="text-lg font-bold text-gray-900">
              {(memory.importance * 100).toFixed(0)}%
            </div>
          </div>
          <div>
            <div className="flex items-center gap-1 text-sm text-gray-500 mb-1">
              <Eye className="w-4 h-4" />
              Strength
            </div>
            <div className="text-lg font-bold text-gray-900">
              {(strength * 100).toFixed(0)}%
            </div>
          </div>
          <div>
            <div className="flex items-center gap-1 text-sm text-gray-500 mb-1">
              <Eye className="w-4 h-4" />
              Access Count
            </div>
            <div className="text-lg font-bold text-gray-900">
              {memory.access_count}
            </div>
          </div>
        </div>

        {/* Tags */}
        {memory.tags && memory.tags.length > 0 && (
          <div className="px-6 py-4 border-t border-gray-200">
            <div className="flex items-center gap-2">
              <Tag className="w-4 h-4 text-gray-400" />
              <div className="flex gap-2">
                {memory.tags.map((tag: string) => (
                  <span key={tag} className="px-2 py-1 text-xs bg-indigo-100 text-indigo-700 rounded">
                    {tag}
                  </span>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* Timestamps */}
        <div className="px-6 py-4 border-t border-gray-200 bg-gray-50">
          <div className="flex items-center gap-4 text-sm text-gray-500">
            <div className="flex items-center gap-1">
              <Calendar className="w-4 h-4" />
              Created: {format(new Date(memory.created_at), 'PPpp')}
            </div>
            <div className="flex items-center gap-1">
              <Eye className="w-4 h-4" />
              Last accessed: {formatDistanceToNow(new Date(memory.last_accessed), { addSuffix: true })}
            </div>
          </div>
        </div>
      </div>

      {/* Links */}
      {links && links.length > 0 && (
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center gap-2 mb-4">
            <Link2 className="w-5 h-5 text-indigo-600" />
            <h3 className="text-lg font-semibold">Related Links ({links.length})</h3>
          </div>
          <div className="space-y-3">
            {links.map((link: any) => (
              <div key={link.id} className="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
                <span className={`px-2 py-1 text-xs font-medium rounded ${
                  link.source_id === memory.id ? 'bg-blue-100 text-blue-800' : 'bg-green-100 text-green-800'
                }`}>
                  {link.source_id === memory.id ? 'FROM' : 'TO'}
                </span>
                <span className="px-2 py-1 text-xs font-medium rounded bg-gray-100 text-gray-800">
                  {link.link_type}
                </span>
                <span className="text-sm text-gray-500">
                  Strength: {(link.strength * 100).toFixed(0)}%
                </span>
                <Link
                  to={`/memories/${link.source_id === memory.id ? link.target_id : link.source_id}`}
                  className="text-sm text-indigo-600 hover:text-indigo-800"
                >
                  View related memory
                </Link>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Source */}
      {memory.source && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-sm font-medium text-gray-500 mb-2">Source</h3>
          <p className="text-gray-900">{memory.source}</p>
        </div>
      )}

      {/* Metadata */}
      {memory.metadata && Object.keys(memory.metadata).length > 0 && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-sm font-medium text-gray-500 mb-2">Metadata</h3>
          <pre className="text-sm text-gray-900 overflow-auto bg-gray-50 p-4 rounded">
            {JSON.stringify(memory.metadata, null, 2)}
          </pre>
        </div>
      )}
    </div>
  )
}