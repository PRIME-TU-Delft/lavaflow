export class SmoothParameters {
	id: string = Math.random().toString(36);
	name: Readonly<string>;
	layer?: number;
	strength?: number;
	coverage?: number;
	svc_weight?: number;
	allow_svc_change?: boolean;
	altitude?: number;
	altitude_group?: number;
	percentage_of_altitude_step?: number;

	constructor(
		name: string,
		layer?: number,
		strength?: number,
		coverage?: number,
		svc_weight?: number,
		allow_svc_change?: boolean,
		altitude?: number,
		altitude_group?: number,
		percentage_of_altitude_step?: number
	) {
		this.name = name;
		this.layer = layer;
		this.strength = strength;
		this.coverage = coverage;
		this.svc_weight = svc_weight;
		this.allow_svc_change = allow_svc_change;
		this.altitude = altitude;
		this.altitude_group = altitude_group;
		this.percentage_of_altitude_step = percentage_of_altitude_step;
	}

	static create(name: string): SmoothParameters {
		if (name == 'None') throw new Error('SmoothParameters.create: name == "None is not allowed"');

		switch (name) {
			case 'ApplySmoothToLayer':
				return ApplySmoothToLayer.default();
			case 'SetAltitudeForLayer':
				return SetAltitudeForLayer.default();
			case 'ApplySmoothToAltitudeGroup':
				return ApplySmoothToAltitudeGroup.default();
			case 'IncreaseAltitudeForAltitudeGroup':
				return IncreaseAltitudeForAltitudeGroup.default();
			case 'ApplySmoothToMiddleLayers':
				return ApplySmoothToMiddleLayers.default();
			case 'ApplySmoothToMountainTop':
				return ApplySmoothToMountainTop.default();
		}

		throw new Error(`SmoothParameters.create: unknown name: ${name}`);
	}

	setValue(key: string, value: any) {
		if (!(key in this)) return;

		// @ts-ignore
		this[key] = value;
	}

	getValue(key: string) {
		if (!(key in this)) return;

		// @ts-ignore
		return this[key];
	}

	toArray() {
		return Object.entries(this).filter(
			([key, value]) => value !== undefined && key != 'name' && key != 'id'
		);
	}

	toString() {
		return (
			this.name +
			' : ' +
			this.toArray()
				.map(([key, value]) => `${key}: ${value}`)
				.join(', ')
		);
	}
}

export class ApplySmoothToLayer extends SmoothParameters {
	constructor(
		layer: number,
		strength: number,
		coverage: number,
		svc_weight: number,
		allow_svc_change: boolean
	) {
		super(
			'ApplySmoothToLayer',
			layer,
			strength,
			coverage,
			svc_weight,
			allow_svc_change,
			undefined,
			undefined,
			undefined
		);
	}

	static default() {
		return new ApplySmoothToLayer(0, 0.5, 0.5, 0.5, true);
	}
}

export class SetAltitudeForLayer extends SmoothParameters {
	constructor(layer: number, altitude: number, allow_svc_change: boolean) {
		super(
			'SetAltitudeForLayer',
			layer,
			undefined,
			undefined,
			undefined,
			allow_svc_change,
			altitude,
			undefined,
			undefined
		);
	}

	static default() {
		return new SetAltitudeForLayer(0, 0, true);
	}
}

export class ApplySmoothToAltitudeGroup extends SmoothParameters {
	constructor(
		altitude_group: number,
		strength: number,
		coverage: number,
		svc_weight: number,
		allow_svc_change: boolean
	) {
		super(
			'ApplySmoothToAltitudeGroup',
			undefined,
			strength,
			coverage,
			svc_weight,
			allow_svc_change,
			undefined,
			altitude_group,
			undefined
		);
	}

	static default() {
		return new ApplySmoothToAltitudeGroup(0, 0.5, 0.5, 0.5, true);
	}
}

export class IncreaseAltitudeForAltitudeGroup extends SmoothParameters {
	constructor(
		altitude_group: number,
		percentage_of_altitude_step: number,
		allow_svc_change: boolean
	) {
		super(
			'IncreaseAltitudeForAltitudeGroup',
			undefined,
			undefined,
			undefined,
			undefined,
			allow_svc_change,
			undefined,
			altitude_group,
			percentage_of_altitude_step
		);
	}

	static default() {
		return new IncreaseAltitudeForAltitudeGroup(0, 0.5, true);
	}
}

export class ApplySmoothToMiddleLayers extends SmoothParameters {
	constructor(strength: number, coverage: number, svc_weight: number, allow_svc_change: boolean) {
		super(
			'ApplySmoothToMiddleLayers',
			undefined,
			strength,
			coverage,
			svc_weight,
			allow_svc_change,
			undefined,
			undefined,
			undefined
		);
	}

	static default() {
		return new ApplySmoothToMiddleLayers(0.5, 0.5, 0.5, true);
	}
}

export class ApplySmoothToMountainTop extends SmoothParameters {
	constructor(strength: number, coverage: number, svc_weight: number, allow_svc_change: boolean) {
		super(
			'ApplySmoothToMountainTop',
			undefined,
			strength,
			coverage,
			svc_weight,
			allow_svc_change,
			undefined,
			undefined,
			undefined
		);
	}

	static default() {
		return new ApplySmoothToMountainTop(0.5, 0.5, 0.5, true);
	}
}
