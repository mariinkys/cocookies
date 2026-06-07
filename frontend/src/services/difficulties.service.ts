import api from '@/api/axios'
import type { PageResponse, PaginatedParams } from '@/types/common.types'
import type { DifficultyRequest, DifficultyResponse } from '@/types/difficulty.types'

class DifficultiesService {
  async getAll(params?: PaginatedParams): Promise<PageResponse<DifficultyResponse>> {
    const { data } = await api.get<PageResponse<DifficultyResponse>>('/api/difficulties', {
      params,
    })
    return data
  }

  async getSelector(): Promise<DifficultyResponse[]> {
    const { data } = await api.get<DifficultyResponse[]>('/api/difficulties/selector')
    return data
  }

  async getById(id: string): Promise<DifficultyResponse> {
    const { data } = await api.get<DifficultyResponse>(`/api/difficulties/${id}`)
    return data
  }

  async create(payload: DifficultyRequest): Promise<DifficultyResponse> {
    const { data } = await api.post<DifficultyResponse>('/api/difficulties', payload)
    return data
  }

  async update(id: string, payload: DifficultyRequest): Promise<DifficultyResponse> {
    const { data } = await api.put<DifficultyResponse>(`/api/difficulties/${id}`, payload)
    return data
  }

  async delete(id: string): Promise<void> {
    const { data } = await api.delete<void>(`/api/difficulties/${id}`)
    return data
  }
}

export const difficultiesService = new DifficultiesService()
