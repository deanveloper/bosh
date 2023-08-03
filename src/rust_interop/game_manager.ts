import { createContext, createSignal } from 'solid-js';
import { EntityStart, Line, RuntimeEntity, Track } from './tauri_types';
import {
	addEntity,
	addLine,
	entityPositionsAt,
	loadTrack,
	removeLine,
} from './tauri_commands';

export class GameManager {
	#frameSignal = createSignal<number>(0);
	#linesSignal = createSignal<Line[]>([]);
	#entitiesSignal = createSignal<RuntimeEntity[]>([]);

	lines(): Line[] {
		return this.#linesSignal[0]();
	}

	entities(): RuntimeEntity[] {
		return this.#entitiesSignal[0]();
	}

	frame(): number {
		return this.#frameSignal[0]();
	}

	async loadTrack(path: string): Promise<Track> {
		const track = await loadTrack(path);
		this.#setLines(track.lines);

		const entities = await entityPositionsAt(this.frame());
		this.#setEntities(entities);

		return track;
	}

	async setFrame(frame: number) {
		const entities = await entityPositionsAt(frame);
		this.#setEntities(entities);
		this.#setFrame(frame);
	}

	async addLine(line: Line) {
		const lines = await addLine(line);
		this.#setLines(lines);
	}

	async removeLine(line: Line) {
		const lines = await removeLine(line);
		this.#setLines(lines);
	}

	async addEntity(entity: EntityStart) {
		await addEntity(entity);

		await this.setFrame(this.frame());
	}

	#setFrame(frame: number) {
		this.#frameSignal[1](frame);
	}

	#setLines(newLines: Line[]) {
		this.#linesSignal[1](newLines);
	}

	#setEntities(newEntities: RuntimeEntity[]) {
		this.#entitiesSignal[1](newEntities);
	}
}

export const GameContext = createContext<GameManager>(new GameManager());
