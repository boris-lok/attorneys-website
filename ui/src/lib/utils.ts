import { defer, Observable } from 'rxjs';

export function startWithTap<T>(callback: () => void) {
	return (source: Observable<T>) =>
		defer(() => {
			callback();
			return source;
		});
}

export function shuffle<T>(array: T[]): T[] {
	let currentIndex = array.length, randomIndex;

	// While there remain elements to shuffle.
	while (currentIndex != 0) {

		// Pick a remaining element.
		randomIndex = Math.floor(Math.random() * currentIndex);
		currentIndex--;

		// And swap it with the current element.
		[array[currentIndex], array[randomIndex]] = [
			array[randomIndex], array[currentIndex]];
	}

	return array;
}

export function text_overflow(text: string, limit: number) {
	const threeDots = '...';
	if (text.length > limit) {
		return text.substring(0, limit - threeDots.length) + threeDots;
	}
	return text;
}