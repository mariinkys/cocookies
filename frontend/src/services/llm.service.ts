import api from '@/api/axios'
import type { LlmResponse } from '@/types/llm.types'

export const llmService = {
  async analyzeFile(file: File, prompt: string): Promise<LlmResponse> {
    const formData = new FormData()
    formData.append('file', file)
    const dataPart = new Blob([JSON.stringify({ prompt })], {
      type: 'application/json',
    })
    formData.append('data', dataPart)

    const response = await api.post<LlmResponse>('/api/llm/analyze/file', formData)
    return response.data
  },

  async analyzeUrl(url: string, prompt: string): Promise<LlmResponse> {
    const response = await api.post<LlmResponse>(
      '/api/llm/analyze/url',
      { prompt },
      { params: { url } },
    )

    return response.data
  },
}
