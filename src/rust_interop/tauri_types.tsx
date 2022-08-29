export type EntityPoint = {
	previousLocation: [number, number];
	location: [number, number];
	momentum: [number, number];
	friction: number;
};

export type Joint = {
	pair1: [string, string];
	pair2: [string, string];
};

export type BoneType =
	| 'Normal'
	| { Mount: { endurance: number } }
	| { Repel: { lengthFactor: number } };

export type Bone = {
	p1: string;
	p2: string;
	resting_length: number;
	bone_type: BoneType;
};

export type RuntimeEntity = {
	points: Record<string, EntityPoint>;
	bones: Bone[];
	joints: Joint[];
};

export type EntityStart =
	| {
			boshSled: {
				position: [number, number];
				velocity?: [number, number];
			};
	  }
	| {
			custom: RuntimeEntity;
	  };

export type LinePoint = {
	location: [number, number];
	extended?: boolean;
};

export type Line = {
	flipped: boolean;
	lineType: 'Normal' | { Accelerate: { accel: number } } | 'Scenery';
	ends: [LinePoint, LinePoint];
};

export type Track = {
	meta?: Record<string, any>; // todo strong types
	lines: Line[];
	entities: EntityStart[];
};
