/**
 * Animation utilities for Svelte components
 */

import { cubicOut, elasticOut, backOut } from 'svelte/easing';
import type { TransitionConfig } from 'svelte/transition';

/**
 * Check if user prefers reduced motion
 */
export function prefersReducedMotion(): boolean {
	if (typeof window === 'undefined') return false;
	return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

/**
 * Slide in from direction with respect to reduced motion preference
 */
export function slideIn(
	node: Element,
	{
		direction = 'up',
		distance = 50,
		duration = 600,
		delay = 0,
		easing = cubicOut
	}: {
		direction?: 'up' | 'down' | 'left' | 'right';
		distance?: number;
		duration?: number;
		delay?: number;
		easing?: (t: number) => number;
	} = {}
): TransitionConfig {
	if (prefersReducedMotion()) {
		return {
			duration: 0,
			css: () => ''
		};
	}

	const transforms = {
		up: `translateY(${distance}px)`,
		down: `translateY(-${distance}px)`,
		left: `translateX(${distance}px)`,
		right: `translateX(-${distance}px)`
	};

	return {
		delay,
		duration,
		easing,
		css: (t) => {
			const transform = transforms[direction];
			return `
				transform: ${transform};
				opacity: ${t};
			`;
		}
	};
}

/**
 * Fade in with scale animation
 */
export function fadeScale(
	node: Element,
	{
		duration = 400,
		delay = 0,
		start = 0.95,
		easing = cubicOut
	}: {
		duration?: number;
		delay?: number;
		start?: number;
		easing?: (t: number) => number;
	} = {}
): TransitionConfig {
	if (prefersReducedMotion()) {
		return {
			duration: 0,
			css: () => ''
		};
	}

	return {
		delay,
		duration,
		easing,
		css: (t) => `
			transform: scale(${start + (1 - start) * t});
			opacity: ${t};
		`
	};
}

/**
 * Stagger animation for lists
 */
export function stagger(
	node: Element,
	{
		duration = 400,
		delay = 0,
		staggerDelay = 100,
		easing = cubicOut
	}: {
		duration?: number;
		delay?: number;
		staggerDelay?: number;
		easing?: (t: number) => number;
	} = {}
): TransitionConfig {
	if (prefersReducedMotion()) {
		return {
			duration: 0,
			css: () => ''
		};
	}

	const index = Array.from(node.parentElement?.children || []).indexOf(node);
	const totalDelay = delay + (index * staggerDelay);

	return {
		delay: totalDelay,
		duration,
		easing,
		css: (t) => `
			transform: translateY(${30 * (1 - t)}px);
			opacity: ${t};
		`
	};
}

/**
 * Bounce in animation
 */
export function bounceIn(
	node: Element,
	{
		duration = 600,
		delay = 0,
		easing = elasticOut
	}: {
		duration?: number;
		delay?: number;
		easing?: (t: number) => number;
	} = {}
): TransitionConfig {
	if (prefersReducedMotion()) {
		return {
			duration: 0,
			css: () => ''
		};
	}

	return {
		delay,
		duration,
		easing,
		css: (t) => `
			transform: scale(${t});
			opacity: ${t};
		`
	};
}

/**
 * Slide and fade animation
 */
export function slideFade(
	node: Element,
	{
		direction = 'up',
		distance = 30,
		duration = 400,
		delay = 0,
		easing = cubicOut
	}: {
		direction?: 'up' | 'down' | 'left' | 'right';
		distance?: number;
		duration?: number;
		delay?: number;
		easing?: (t: number) => number;
	} = {}
): TransitionConfig {
	if (prefersReducedMotion()) {
		return {
			duration: 0,
			css: () => ''
		};
	}

	const transforms = {
		up: `translateY(${distance}px)`,
		down: `translateY(-${distance}px)`,
		left: `translateX(${distance}px)`,
		right: `translateX(-${distance}px)`
	};

	return {
		delay,
		duration,
		easing,
		css: (t) => `
			transform: ${transforms[direction]};
			opacity: ${t};
		`
	};
}

/**
 * Page transition animation
 */
export function pageTransition(
	node: Element,
	{
		duration = 300,
		easing = cubicOut
	}: {
		duration?: number;
		easing?: (t: number) => number;
	} = {}
): TransitionConfig {
	if (prefersReducedMotion()) {
		return {
			duration: 0,
			css: () => ''
		};
	}

	return {
		duration,
		easing,
		css: (t) => `
			transform: translateX(${(1 - t) * 20}px);
			opacity: ${t};
		`
	};
}

/**
 * Reveal animation for scroll-triggered elements
 */
export function reveal(
	node: Element,
	{
		threshold = 0.1,
		rootMargin = '0px',
		duration = 600,
		delay = 0,
		easing = cubicOut
	}: {
		threshold?: number;
		rootMargin?: string;
		duration?: number;
		delay?: number;
		easing?: (t: number) => number;
	} = {}
) {
	if (prefersReducedMotion()) {
		return {
			destroy() {}
		};
	}

	let isVisible = false;
	
	const observer = new IntersectionObserver(
		(entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting && !isVisible) {
					isVisible = true;
					node.dispatchEvent(new CustomEvent('reveal'));
					observer.unobserve(node);
				}
			});
		},
		{ threshold, rootMargin }
	);

	observer.observe(node);

	return {
		destroy() {
			observer.disconnect();
		}
	};
}

/**
 * Smooth scroll to element
 */
export function smoothScrollTo(
	element: Element | string,
	options: ScrollIntoViewOptions = {}
) {
	const target = typeof element === 'string' 
		? document.querySelector(element)
		: element;

	if (target) {
		target.scrollIntoView({
			behavior: 'smooth',
			block: 'start',
			...options
		});
	}
}

/**
 * Parallax scroll effect
 */
export function parallax(
	node: Element,
	{
		speed = 0.5,
		offset = 0
	}: {
		speed?: number;
		offset?: number;
	} = {}
) {
	if (prefersReducedMotion()) {
		return {
			destroy() {}
		};
	}

	let ticking = false;

	function updateTransform() {
		const rect = node.getBoundingClientRect();
		const scrolled = window.pageYOffset;
		const rate = scrolled * speed;
		
		(node as HTMLElement).style.transform = `translateY(${rate + offset}px)`;
		ticking = false;
	}

	function handleScroll() {
		if (!ticking) {
			requestAnimationFrame(updateTransform);
			ticking = true;
		}
	}

	window.addEventListener('scroll', handleScroll, { passive: true });

	return {
		destroy() {
			window.removeEventListener('scroll', handleScroll);
		}
	};
}

/**
 * Touch gesture utilities
 */
export interface TouchGestureOptions {
	onSwipeLeft?: () => void;
	onSwipeRight?: () => void;
	onSwipeUp?: () => void;
	onSwipeDown?: () => void;
	threshold?: number;
}

export function touchGestures(
	node: Element,
	options: TouchGestureOptions = {}
) {
	const { threshold = 50 } = options;
	let startX = 0;
	let startY = 0;
	let endX = 0;
	let endY = 0;

	function handleTouchStart(event: TouchEvent) {
		const touch = event.touches[0];
		startX = touch.clientX;
		startY = touch.clientY;
	}

	function handleTouchEnd(event: TouchEvent) {
		const touch = event.changedTouches[0];
		endX = touch.clientX;
		endY = touch.clientY;

		const deltaX = endX - startX;
		const deltaY = endY - startY;

		if (Math.abs(deltaX) > Math.abs(deltaY)) {
			// Horizontal swipe
			if (Math.abs(deltaX) > threshold) {
				if (deltaX > 0) {
					options.onSwipeRight?.();
				} else {
					options.onSwipeLeft?.();
				}
			}
		} else {
			// Vertical swipe
			if (Math.abs(deltaY) > threshold) {
				if (deltaY > 0) {
					options.onSwipeDown?.();
				} else {
					options.onSwipeUp?.();
				}
			}
		}
	}

	node.addEventListener('touchstart', handleTouchStart, { passive: true });
	node.addEventListener('touchend', handleTouchEnd, { passive: true });

	return {
		destroy() {
			node.removeEventListener('touchstart', handleTouchStart);
			node.removeEventListener('touchend', handleTouchEnd);
		}
	};
}