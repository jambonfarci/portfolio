import { writable } from 'svelte/store';

export interface Toast {
	id: string;
	type: 'success' | 'error' | 'warning' | 'info';
	title: string;
	message?: string;
	duration?: number;
	dismissible?: boolean;
}

export interface ToastOptions {
	type?: Toast['type'];
	title: string;
	message?: string;
	duration?: number;
	dismissible?: boolean;
}

// Store for active toasts
export const toasts = writable<Toast[]>([]);

// Toast management functions
export const toastStore = {
	// Add a new toast
	add(options: ToastOptions): string {
		const id = Math.random().toString(36).substr(2, 9);
		const toast: Toast = {
			id,
			type: options.type || 'info',
			title: options.title,
			message: options.message,
			duration: options.duration || 5000,
			dismissible: options.dismissible !== false
		};

		toasts.update(currentToasts => [...currentToasts, toast]);

		// Auto-dismiss after duration
		if (toast.duration > 0) {
			setTimeout(() => {
				this.dismiss(id);
			}, toast.duration);
		}

		return id;
	},

	// Dismiss a specific toast
	dismiss(id: string) {
		toasts.update(currentToasts => 
			currentToasts.filter(toast => toast.id !== id)
		);
	},

	// Clear all toasts
	clear() {
		toasts.set([]);
	},

	// Convenience methods for different toast types
	success(title: string, message?: string, duration?: number): string {
		return this.add({ type: 'success', title, message, duration });
	},

	error(title: string, message?: string, duration?: number): string {
		return this.add({ type: 'error', title, message, duration: duration || 8000 });
	},

	warning(title: string, message?: string, duration?: number): string {
		return this.add({ type: 'warning', title, message, duration });
	},

	info(title: string, message?: string, duration?: number): string {
		return this.add({ type: 'info', title, message, duration });
	}
};