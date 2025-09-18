/**
 * Performance monitoring utilities
 */

export interface PerformanceMetrics {
	loadTime: number;
	domContentLoaded: number;
	firstContentfulPaint?: number;
	largestContentfulPaint?: number;
	cumulativeLayoutShift?: number;
	firstInputDelay?: number;
}

/**
 * Measure page load performance
 */
export function measurePagePerformance(): Promise<PerformanceMetrics> {
	return new Promise((resolve) => {
		const metrics: Partial<PerformanceMetrics> = {};
		
		// Basic timing metrics
		window.addEventListener('load', () => {
			const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
			
			metrics.loadTime = navigation.loadEventEnd - navigation.fetchStart;
			metrics.domContentLoaded = navigation.domContentLoadedEventEnd - navigation.fetchStart;
			
			// Web Vitals if available
			if ('PerformanceObserver' in window) {
				// First Contentful Paint
				new PerformanceObserver((list) => {
					const entries = list.getEntries();
					entries.forEach((entry) => {
						if (entry.name === 'first-contentful-paint') {
							metrics.firstContentfulPaint = entry.startTime;
						}
					});
				}).observe({ entryTypes: ['paint'] });
				
				// Largest Contentful Paint
				new PerformanceObserver((list) => {
					const entries = list.getEntries();
					const lastEntry = entries[entries.length - 1];
					metrics.largestContentfulPaint = lastEntry.startTime;
				}).observe({ entryTypes: ['largest-contentful-paint'] });
				
				// Cumulative Layout Shift
				new PerformanceObserver((list) => {
					let clsValue = 0;
					for (const entry of list.getEntries()) {
						if (!(entry as any).hadRecentInput) {
							clsValue += (entry as any).value;
						}
					}
					metrics.cumulativeLayoutShift = clsValue;
				}).observe({ entryTypes: ['layout-shift'] });
				
				// First Input Delay
				new PerformanceObserver((list) => {
					const entries = list.getEntries();
					entries.forEach((entry) => {
						metrics.firstInputDelay = (entry as any).processingStart - entry.startTime;
					});
				}).observe({ entryTypes: ['first-input'] });
			}
			
			// Resolve after a short delay to collect all metrics
			setTimeout(() => {
				resolve(metrics as PerformanceMetrics);
			}, 1000);
		});
	});
}

/**
 * Monitor resource loading performance
 */
export function monitorResourcePerformance() {
	if ('PerformanceObserver' in window) {
		const observer = new PerformanceObserver((list) => {
			const entries = list.getEntries();
			entries.forEach((entry) => {
				if (entry.initiatorType === 'img') {
					const loadTime = entry.responseEnd - entry.startTime;
					console.log(`Image loaded: ${entry.name} in ${loadTime.toFixed(2)}ms`);
					
					// Log slow images (>500ms)
					if (loadTime > 500) {
						console.warn(`Slow image detected: ${entry.name} took ${loadTime.toFixed(2)}ms`);
					}
				}
			});
		});
		
		observer.observe({ entryTypes: ['resource'] });
	}
}

/**
 * Check if connection is slow
 */
export function isSlowConnection(): boolean {
	const connection = (navigator as any).connection;
	if (connection) {
		return connection.effectiveType === 'slow-2g' || 
		       connection.effectiveType === '2g' ||
		       connection.saveData === true;
	}
	return false;
}

/**
 * Adaptive loading based on connection speed
 */
export function getAdaptiveImageQuality(): number {
	if (isSlowConnection()) {
		return 60; // Lower quality for slow connections
	}
	return 80; // Standard quality
}

/**
 * Measure component render time
 */
export function measureRenderTime(componentName: string) {
	const startTime = performance.now();
	
	return {
		end: () => {
			const endTime = performance.now();
			const renderTime = endTime - startTime;
			console.log(`${componentName} rendered in ${renderTime.toFixed(2)}ms`);
			return renderTime;
		}
	};
}

/**
 * Performance budget checker
 */
export interface PerformanceBudget {
	maxLoadTime: number; // milliseconds
	maxImageSize: number; // bytes
	maxBundleSize: number; // bytes
}

export function checkPerformanceBudget(
	metrics: PerformanceMetrics,
	budget: PerformanceBudget
): { passed: boolean; violations: string[] } {
	const violations: string[] = [];
	
	if (metrics.loadTime > budget.maxLoadTime) {
		violations.push(`Load time exceeded: ${metrics.loadTime}ms > ${budget.maxLoadTime}ms`);
	}
	
	if (metrics.largestContentfulPaint && metrics.largestContentfulPaint > 2500) {
		violations.push(`LCP too slow: ${metrics.largestContentfulPaint}ms > 2500ms`);
	}
	
	if (metrics.cumulativeLayoutShift && metrics.cumulativeLayoutShift > 0.1) {
		violations.push(`CLS too high: ${metrics.cumulativeLayoutShift} > 0.1`);
	}
	
	if (metrics.firstInputDelay && metrics.firstInputDelay > 100) {
		violations.push(`FID too slow: ${metrics.firstInputDelay}ms > 100ms`);
	}
	
	return {
		passed: violations.length === 0,
		violations
	};
}