import { invoke } from '@tauri-apps/api/tauri';
import { Entity, Line, RealEntity } from './tauri_types';

export async function entityPositionsAt(frame: number): Promise<RealEntity[]> {
	return (await invoke('entity_positions_at', { frame })) as RealEntity[];
}

export async function addEntity(entity: Entity): Promise<void> {
	await invoke('add_entity', { entity });
}

export async function removeEntity(entity: Entity): Promise<void> {
	await invoke('remove_entity', { entity });
}

export async function addLine(line: Line): Promise<Line[]> {
	return await invoke('add_line', { line });
}

export async function clear(): Promise<void> {
	await invoke('clear', {});
}
