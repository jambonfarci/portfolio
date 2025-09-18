/**
 * Responsive design utilities
 */

export interface Breakpoints {
	sm: number;
	md: number;
	lg: number;
	xl: number;
	'2xl': number;
}

export const breakpoints: Breakpoints = {
	sm: 640,
	md: 768,
	lg: 1024,
	xl: 1280,
	'2xl': 1536
};

/**
 * Get current screen size category
 */
export function getScreenSize(): keyof Breakpoints | 'xs' {
	if (typeof window === 'undefined') return 'lg';
	
	const width = window.innerWidth;
	
	if (width >= breakpoints['2xl']) return '2xl';
	if (width >= breakpoints.xl) return 'xl';
	if (width >= breakpoints.lg) return 'lg';
	if (width >= breakpoints.md) return 'md';
	if (width >= breakpoints.sm) return 'sm';
	return 'xs';
}

/**
 * Check if screen is mobile size
 */
export function isMobile(): boolean {
	if (typeof window === 'undefined') return false;
	return window.innerWidth < breakpoints.md;
}

/**
 * Check if screen is tablet size
 */
export function isTablet(): boolean {
	if (typeof window === 'undefined') return false;
	const width = window.innerWidth;
	return width >= breakpoints.md && width < breakpoints.lg;
}

/**
 * Check if screen is desktop size
 */
export function isDesktop(): boolean {
	if (typeof window === 'undefined') return true;
	return window.innerWidth >= breakpoints.lg;
}

/**
 * Media query matcher
 */
export function matchMedia(query: string): boolean {
	if (typeof window === 'undefined') return false;
	return window.matchMedia(query).matches;
}

/**
 * Responsive value selector
 */
export function responsive<T>(values: {
	xs?: T;
	sm?: T;
	md?: T;
	lg?: T;
	xl?: T;
	'2xl'?: T;
	default: T;
}): T {
	const screenSize = getScreenSize();
	
	// Return the value for current screen size or fall back to smaller sizes
	if (screenSize === '2xl' && values['2xl'] !== undefined) return values['2xl'];
	if ((screenSize === '2xl' || screenSize === 'xl') && values.xl !== undefined) return values.xl;
	if ((screenSize === '2xl' || screenSize === 'xl' || screenSize === 'lg') && values.lg !== undefined) return values.lg;
	if ((screenSize !== 'xs' && screenSize !== 'sm') && values.md !== undefined) return values.md;
	if (screenSize !== 'xs' && values.sm !== undefined) return values.sm;
	if (values.xs !== undefined) return values.xs;
	
	return values.default;
}

/**
 * Create a reactive store for screen size
 */
export function createScreenSizeStore() {
	if (typeof window === 'undefined') {
		return {
			subscribe: (callback: (value: any) => void) => {
				callback({ size: 'lg', isMobile: false, isTablet: false, isDesktop: true });
				return () => {};
			}
		};
	}

	let subscribers: Array<(value: any) => void> = [];
	let currentValue = {
		size: getScreenSize(),
		isMobile: isMobile(),
		isTablet: isTablet(),
		isDesktop: isDesktop()
	};

	function updateValue() {
		const newValue = {
			size: getScreenSize(),
			isMobile: isMobile(),
			isTablet: isTablet(),
			isDesktop: isDesktop()
		};

		if (JSON.stringify(newValue) !== JSON.stringify(currentValue)) {
			currentValue = newValue;
			subscribers.forEach(callback => callback(currentValue));
		}
	}

	window.addEventListener('resize', updateValue);

	return {
		subscribe(callback: (value: any) => void) {
			subscribers.push(callback);
			callback(currentValue);

			return () => {
				subscribers = subscribers.filter(sub => sub !== callback);
				if (subscribers.length === 0) {
					window.removeEventListener('resize', updateValue);
				}
			};
		}
	};
}

/**
 * Touch device detection
 */
export function isTouchDevice(): boolean {
	if (typeof window === 'undefined') return false;
	
	return 'ontouchstart' in window ||
		navigator.maxTouchPoints > 0 ||
		(navigator as any).msMaxTouchPoints > 0;
}

/**
 * Get optimal grid columns based on screen size
 */
export function getGridColumns(
	items: number,
	maxColumns: { xs?: number; sm?: number; md?: number; lg?: number; xl?: number } = {}
): number {
	const screenSize = getScreenSize();
	const defaults = { xs: 1, sm: 2, md: 2, lg: 3, xl: 4 };
	const config = { ...defaults, ...maxColumns };
	
	let columns = config.xs || 1;
	
	if (screenSize === 'sm') columns = config.sm || 2;
	else if (screenSize === 'md') columns = config.md || 2;
	else if (screenSize === 'lg') columns = config.lg || 3;
	else if (screenSize === 'xl' || screenSize === '2xl') columns = config.xl || 4;
	
	return Math.min(columns, items);
}

/**
 * Container padding based on screen size
 */
export function getContainerPadding(): string {
	return responsive({
		xs: 'px-4',
		sm: 'px-6',
		lg: 'px-8',
		default: 'px-4'
	});
}

/**
 * Text size based on screen size
 */
export function getResponsiveTextSize(base: string): string {
	const sizeMap: Record<string, Record<string, string>> = {
		'text-xs': { xs: 'text-xs', sm: 'text-sm', md: 'text-sm', lg: 'text-base' },
		'text-sm': { xs: 'text-sm', sm: 'text-base', md: 'text-base', lg: 'text-lg' },
		'text-base': { xs: 'text-base', sm: 'text-lg', md: 'text-lg', lg: 'text-xl' },
		'text-lg': { xs: 'text-lg', sm: 'text-xl', md: 'text-xl', lg: 'text-2xl' },
		'text-xl': { xs: 'text-xl', sm: 'text-2xl', md: 'text-2xl', lg: 'text-3xl' },
		'text-2xl': { xs: 'text-2xl', sm: 'text-3xl', md: 'text-3xl', lg: 'text-4xl' },
		'text-3xl': { xs: 'text-3xl', sm: 'text-4xl', md: 'text-4xl', lg: 'text-5xl' },
		'text-4xl': { xs: 'text-4xl', sm: 'text-5xl', md: 'text-5xl', lg: 'text-6xl' }
	};

	const screenSize = getScreenSize();
	return sizeMap[base]?.[screenSize] || base;
}

/**
 * Spacing based on screen size
 */
export function getResponsiveSpacing(base: number): string {
	return responsive({
		xs: `${base}`,
		sm: `${base * 1.25}`,
		md: `${base * 1.5}`,
		lg: `${base * 2}`,
		default: `${base}`
	});
}