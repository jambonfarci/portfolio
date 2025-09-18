/**
 * Lazy loading utility for Svelte components and content
 */

export interface LazyLoadOptions {
	rootMargin?: string;
	threshold?: number;
	once?: boolean;
}

/**
 * Svelte action for lazy loading elements
 */
export function lazyLoad(
	node: HTMLElement,
	options: LazyLoadOptions = {}
) {
	const { rootMargin = '50px', threshold = 0, once = true } = options;
	
	let observer: IntersectionObserver;
	
	const handleIntersection = (entries: IntersectionObserverEntry[]) => {
		entries.forEach((entry) => {
			if (entry.isIntersecting) {
				node.dispatchEvent(new CustomEvent('lazyload'));
				
				if (once) {
					observer.unobserve(node);
				}
			}
		});
	};
	
	observer = new IntersectionObserver(handleIntersection, {
		rootMargin,
		threshold
	});
	
	observer.observe(node);
	
	return {
		destroy() {
			if (observer) {
				observer.disconnect();
			}
		}
	};
}

/**
 * Preload critical resources
 */
export function preloadResource(href: string, as: string = 'image') {
	const link = document.createElement('link');
	link.rel = 'preload';
	link.as = as;
	link.href = href;
	document.head.appendChild(link);
}

/**
 * Lazy load images with intersection observer
 */
export function lazyLoadImages(selector: string = 'img[data-lazy]') {
	const images = document.querySelectorAll(selector);
	
	if ('IntersectionObserver' in window) {
		const imageObserver = new IntersectionObserver((entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					const img = entry.target as HTMLImageElement;
					const src = img.dataset.lazy;
					
					if (src) {
						img.src = src;
						img.classList.remove('lazy');
						imageObserver.unobserve(img);
					}
				}
			});
		});
		
		images.forEach((img) => imageObserver.observe(img));
	} else {
		// Fallback for browsers without IntersectionObserver
		images.forEach((img) => {
			const element = img as HTMLImageElement;
			const src = element.dataset.lazy;
			if (src) {
				element.src = src;
			}
		});
	}
}

/**
 * Debounce function for performance optimization
 */
export function debounce<T extends (...args: any[]) => any>(
	func: T,
	wait: number
): (...args: Parameters<T>) => void {
	let timeout: NodeJS.Timeout;
	
	return (...args: Parameters<T>) => {
		clearTimeout(timeout);
		timeout = setTimeout(() => func.apply(null, args), wait);
	};
}

/**
 * Check if user prefers reduced motion
 */
export function prefersReducedMotion(): boolean {
	return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

/**
 * Get optimal image size based on container and device pixel ratio
 */
export function getOptimalImageSize(
	containerWidth: number,
	maxWidth: number = 1920
): number {
	const dpr = window.devicePixelRatio || 1;
	const targetWidth = Math.min(containerWidth * dpr, maxWidth);
	
	// Round to nearest standard size
	const standardSizes = [320, 640, 768, 1024, 1280, 1920];
	return standardSizes.find(size => size >= targetWidth) || maxWidth;
}