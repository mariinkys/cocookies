import api from '@/api/axios'
import type { PageResponse, PaginatedParams } from '@/types/common.types'
import type { CategoryRequest, CategoryResponse } from '@/types/category.types'

class CategoriesService {
  async getAll(params?: PaginatedParams): Promise<PageResponse<CategoryResponse>> {
    const { data } = await api.get<PageResponse<CategoryResponse>>('/api/categories', { params })
    return data
  }

  async getSelector(): Promise<CategoryResponse[]> {
    const { data } = await api.get<CategoryResponse[]>('/api/categories/selector')
    return data
  }

  async getById(id: string): Promise<CategoryResponse> {
    const { data } = await api.get<CategoryResponse>(`/api/categories/${id}`)
    return data
  }

  async create(payload: CategoryRequest): Promise<CategoryResponse> {
    const { data } = await api.post<CategoryResponse>('/api/categories', payload)
    return data
  }

  async update(id: string, payload: CategoryRequest): Promise<CategoryResponse> {
    const { data } = await api.put<CategoryResponse>(`/api/categories/${id}`, payload)
    return data
  }

  async delete(id: string): Promise<void> {
    const { data } = await api.delete<void>(`/api/categories/${id}`)
    return data
  }
}

export const categoriesService = new CategoriesService()
