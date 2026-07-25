import type { Event } from '@/api/types'
import { type Ref, ref } from 'vue'

export async function checkStatus (): Promise<Event> {
    const response = await fetch('http://localhost:80/api/status')
    const data = await response.json()
    return data as Event
}

export function connectWebSocket () {
    const socket = new WebSocket('ws://localhost:80/ws')

    const status: Ref<Event | undefined> = ref(undefined)
    socket.addEventListener('open', () => {
        console.log('WebSocket connected')
    })

    socket.addEventListener('message', event => {
        const data = JSON.parse(event.data)
        status.value = data as Event
    })

    return status
}
