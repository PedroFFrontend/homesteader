type GeospatialLocation = {
	latitude: number;
	longitude: number;
};

export class User {
	location: GeospatialLocation | undefined;
	usdaZone: number | undefined;

	constructor() {}

	setLocation(latitude: number, longitude: number) {
		this.location = { latitude, longitude };
	}
}

const user = new User();
export default user;
