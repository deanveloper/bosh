import { invoke } from '@tauri-apps/api/tauri';
import { EntityStart, Line, RuntimeEntity, Track } from './tauri_types';

export async function entityPositionsAt(
	frame: number,
): Promise<RuntimeEntity[]> {
	return (await invoke('entity_positions_at', { frame })) as RuntimeEntity[];
}

export async function addEntity(entity: EntityStart): Promise<void> {
	await invoke('add_entity', { entity });
}

export async function removeEntity(entity: EntityStart): Promise<void> {
	await invoke('remove_entity', { entity });
}

export async function addLine(line: Line): Promise<Line[]> {
	return await invoke('add_line', { line });
}

export async function removeLine(line: Line): Promise<Line[]> {
	return await invoke('remove_line', { line });
}

export async function loadTrack(path: string): Promise<Track> {
	return await invoke('load_track', { path });
}

export async function clear(): Promise<void> {
	await invoke('clear', {});
}
