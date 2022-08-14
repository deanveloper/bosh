import { invoke } from '@tauri-apps/api/tauri';

export type PointIndex =
	| 'BoshLeftFoot'
	| 'BoshRightFoot'
	| 'BoshLeftHand'
	| 'BoshRightHand'
	| 'BoshShoulder'
	| 'BoshButt'
	| 'SledPeg'
	| 'SledTail'
	| 'SledNose'
	| 'SledRope';

export type Entity = {
	entityType: 'Bosh' | 'Sled' | 'BoshSled';
	points?: Record<PointIndex, [number, number]>;
};

export type Line = {
	flipped: boolean;
	lineType: 'Normal' | { Accelerate: { accel: number } } | 'Scenery';
	ends: [[number, number], [number, number]];
};

export async function entityPositionsAt(frame: number): Promise<Entity[]> {
	return (await invoke('entity_positions_at', { frame })) as Entity[];
}

export async function addEntity(entity: Entity): Promise<void> {
	await invoke('add_entity', { jsEntity: entity });
}

export async function removeEntity(entity: Entity): Promise<void> {
	await invoke('remove_entity', { jsEntity: entity });
}

export async function addLine(line: Line): Promise<Line[]> {
	return await invoke('add_line', { line });
}

export async function clear(): Promise<void> {
	await invoke('clear', {});
}
