export interface Event {
    event_type: EventType
    memory_info: MemoryInfo
    cpu_info: CpuInfo
}

export enum EventType {
    UPDATE = 'UPDATE',
}

export interface CpuInfo {
    percentage: number
}

export interface MemoryInfo {
    used_memory: number
    total_memory: number
}
