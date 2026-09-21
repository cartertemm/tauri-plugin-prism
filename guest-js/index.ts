import { invoke } from '@tauri-apps/api/core'

export interface Features {
	voice: boolean
	rate: boolean
	pitch: boolean
	volume: boolean
}

export interface Info {
	available: boolean
	backend: string | null
	features: Features
}

export interface Voice {
	id: number
	name: string
	language: string | null
}

export function speak(text: string, interrupt = false): Promise<void> {
	return invoke('plugin:prism|speak', { text, interrupt })
}

export function stop(): Promise<void> {
	return invoke('plugin:prism|stop')
}

export function info(): Promise<Info> {
	return invoke('plugin:prism|info')
}

export function voices(): Promise<Voice[]> {
	return invoke('plugin:prism|voices')
}

export function setVoice(id: number): Promise<void> {
	return invoke('plugin:prism|set_voice', { id })
}

export function setRate(value: number): Promise<void> {
	return invoke('plugin:prism|set_rate', { value })
}

export function setPitch(value: number): Promise<void> {
	return invoke('plugin:prism|set_pitch', { value })
}

export function setVolume(value: number): Promise<void> {
	return invoke('plugin:prism|set_volume', { value })
}
