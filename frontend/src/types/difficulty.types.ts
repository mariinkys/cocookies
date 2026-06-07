export interface DifficultyRequest {
  name: string
  description: string | null
}

export interface DifficultyResponse {
  id: string
  userId: string
  name: string
  description: string | null
  createdAt: string
  updatedAt: string
}
