import { v4 as uuid } from 'uuid';

export class Alert {
	id: string;
	severity: 'info' | 'severe' | 'critical';
	description: string;
	suggestedAction: string;

	constructor(
		severity: 'info' | 'severe' | 'critical',
		description: string,
		suggestedAction: string
	) {
		this.id = uuid();
		this.severity = severity;
		this.description = description;
		this.suggestedAction = suggestedAction;
	}

	dismiss() {
		const alertManager = new AlertManager();
		alertManager.alerts = alertManager.alerts.filter((a) => a.id !== this.id);
	}
}

class AlertManager {
	private static _instance: AlertManager | undefined;
	alerts: Alert[] = $state([]);

	constructor() {
		if (AlertManager._instance) return AlertManager._instance;
		AlertManager._instance = this;

		this.alerts = [
			new Alert('critical', 'Soil moisture low in bed A (38%)', 'Check irrigation'),
			new Alert('severe', 'Next 2 days: temperature above average', 'Watch greenhouse'),
			new Alert('info', 'Soil moisture low in bed A (38%)', 'Check irrigation')
		];
	}
}

export const alertManager = new AlertManager();
