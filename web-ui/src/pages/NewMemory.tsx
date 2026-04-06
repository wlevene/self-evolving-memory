import { useState } from 'react'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { useNavigate } from 'react-router-dom'
import { api } from '../api/client'
import { Brain, Save, X, Tag } from 'lucide-react'

export function NewMemory() {
  const navigate = useNavigate()
  const queryClient = useQueryClient()

  const [content, setContent] = useState('')
  const [pool, setPool] = useState('explicit')
  const [type, setType] = useState('fact')
  const [confidence, setConfidence] = useState(0.8)
  const [importance, setImportance] = useState(0.5)
  const [tags, setTags] = useState('')
  const [source, setSource] = useState('')

  const createMutation = useMutation({
    mutationFn: async () => {
      const response = await api.post('/memories', {
        content,
        pool,
        type,
        confidence,
        importance,
        tags: tags.split(',').map(t => t.trim()).filter(Boolean),
        source: source || undefined,
        metadata: {},
      })
      return response.data
    },
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: ['memories'] })
      queryClient.invalidateQueries({ queryKey: ['stats'] })
      navigate(`/memories/${data.id}`)
    },
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (content.trim()) {
      createMutation.mutate()
    }
  }

  return (
    <div className="max-w-2xl mx-auto">
      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex items-center gap-2 mb-6">
          <Brain className="w-6 h-6 text-indigo-600" />
          <h2 className="text-lg font-semibold">Create New Memory</h2>
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Content */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Content *
            </label>
            <textarea
              value={content}
              onChange={(e) => setContent(e.target.value)}
              rows={5}
              placeholder="Enter the memory content..."
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 resize-none"
              required
            />
          </div>

          {/* Pool & Type */}
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Pool
              </label>
              <select
                value={pool}
                onChange={(e) => setPool(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
              >
                <option value="explicit">Explicit (consciously recalled)</option>
                <option value="implicit">Implicit (patterns/preferences)</option>
              </select>
              <p className="mt-1 text-xs text-gray-500">
                Explicit: facts, events, procedures
                <br />
                Implicit: patterns, preferences, behaviors
              </p>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Type
              </label>
              <select
                value={type}
                onChange={(e) => setType(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
              >
                <option value="fact">Fact (declarative knowledge)</option>
                <option value="event">Event (experiences)</option>
                <option value="procedure">Procedure (skills/methods)</option>
                <option value="concept">Concept (abstractions)</option>
                <option value="preference">Preference (user preferences)</option>
                <option value="context">Context (current state)</option>
              </select>
            </div>
          </div>

          {/* Confidence & Importance */}
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Confidence
              </label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.1"
                value={confidence}
                onChange={(e) => setConfidence(Number(e.target.value))}
                className="w-full"
              />
              <div className="text-xs text-gray-500 text-center">
                {(confidence * 100).toFixed(0)}% - How certain is this memory?
              </div>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Importance
              </label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.1"
                value={importance}
                onChange={(e) => setImportance(Number(e.target.value))}
                className="w-full"
              />
              <div className="text-xs text-gray-500 text-center">
                {(importance * 100).toFixed(0)}% - How important is this memory?
              </div>
            </div>
          </div>

          {/* Tags */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <Tag className="w-4 h-4 inline mr-1" />
              Tags (comma-separated)
            </label>
            <input
              type="text"
              value={tags}
              onChange={(e) => setTags(e.target.value)}
              placeholder="e.g., work, project, important"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
            />
          </div>

          {/* Source */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Source (optional)
            </label>
            <input
              type="text"
              value={source}
              onChange={(e) => setSource(e.target.value)}
              placeholder="e.g., conversation, document, observation"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
            />
          </div>

          {/* Actions */}
          <div className="flex items-center justify-end gap-3 pt-4 border-t border-gray-200">
            <button
              type="button"
              onClick={() => navigate('/memories')}
              className="px-4 py-2 text-gray-600 hover:text-gray-900 flex items-center gap-1"
            >
              <X className="w-4 h-4" />
              Cancel
            </button>
            <button
              type="submit"
              disabled={!content.trim() || createMutation.isPending}
              className="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:bg-gray-300 disabled:cursor-not-allowed flex items-center gap-1"
            >
              <Save className="w-4 h-4" />
              {createMutation.isPending ? 'Creating...' : 'Create Memory'}
            </button>
          </div>

          {/* Error */}
          {createMutation.isError && (
            <div className="p-4 bg-red-50 border border-red-200 rounded-md text-red-700 text-sm">
              Error creating memory. Please try again.
            </div>
          )}
        </form>
      </div>
    </div>
  )
}