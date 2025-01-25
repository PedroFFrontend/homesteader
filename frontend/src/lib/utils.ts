import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export const BACKEND_URL = 'http://localhost:8080';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function kelvinToCelsius(kelvin: number) {
	return kelvin - 273.15;
}
