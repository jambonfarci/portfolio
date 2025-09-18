/**
 * Error logging utility for the portfolio application
 * Filters out IDE-related errors and provides structured logging
 */

export interface ErrorContext {
	component?: string;
	action?: string;
	userId?: string;
	timestamp?: Date;
	userAgent?: string;
	url?: string;
}

export interface LoggedError {
	message: string;
	stack?: string;
	context?: ErrorContext;
	level: 'error' | 'warning' | 'info' | 'debug';
	timestamp: Date;
}

class ErrorLogger {
	private logs: LoggedError[] = [];
	private maxLogs = 100; // Keep only the last 100 logs

	// Patterns for errors that should be ignored (IDE-related)
	private ignoredPatterns = [
		'Failed to initialize messaging',
		'tx_attempts_exceeded',
		'tx_ack_timeout',
		'host-additional-hooks',
		'getSingleton',
		'connect',
		'kiro-ide',
		'chrome-extension'
	];

	/**
	 * Check if an error should be ignored
	 */
	private shouldIgnoreError(error: any): boolean {
		const errorMessage = error?.message || error?.reason?.message || String(error);
		return this.ignoredPatterns.some(pattern => 
			errorMessage.toLowerCase().includes(pattern.toLowerCase())
		);
	}

	/**
	 * Log an error with context
	 */
	logError(error: any, context?: ErrorContext): void {
		if (this.shouldIgnoreError(error)) {
			console.debug('Ignored IDE-related error:', error?.message || error);
			return;
		}

		const logEntry: LoggedError = {
			message: error?.message || String(error),
			stack: error?.stack,
			context: {
				...context,
				timestamp: new Date(),
				userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : 'unknown',
				url: typeof window !== 'undefined' ? window.location.href : 'unknown'
			},
			level: 'error',
			timestamp: new Date()
		};

		this.logs.push(logEntry);
		
		// Keep only the last maxLogs entries
		if (this.logs.length > this.maxLogs) {
			this.logs = this.logs.slice(-this.maxLogs);
		}

		// Log to console with structured format
		console.error('Application Error:', {
			message: logEntry.message,
			context: logEntry.context,
			stack: logEntry.stack
		});
	}

	/**
	 * Log a warning
	 */
	logWarning(message: string, context?: ErrorContext): void {
		const logEntry: LoggedError = {
			message,
			context: {
				...context,
				timestamp: new Date(),
				url: typeof window !== 'undefined' ? window.location.href : 'unknown'
			},
			level: 'warning',
			timestamp: new Date()
		};

		this.logs.push(logEntry);
		
		if (this.logs.length > this.maxLogs) {
			this.logs = this.logs.slice(-this.maxLogs);
		}

		console.warn('Application Warning:', {
			message: logEntry.message,
			context: logEntry.context
		});
	}

	/**
	 * Log an info message
	 */
	logInfo(message: string, context?: ErrorContext): void {
		const logEntry: LoggedError = {
			message,
			context: {
				...context,
				timestamp: new Date(),
				url: typeof window !== 'undefined' ? window.location.href : 'unknown'
			},
			level: 'info',
			timestamp: new Date()
		};

		this.logs.push(logEntry);
		
		if (this.logs.length > this.maxLogs) {
			this.logs = this.logs.slice(-this.maxLogs);
		}

		console.info('Application Info:', {
			message: logEntry.message,
			context: logEntry.context
		});
	}

	/**
	 * Get all logged errors
	 */
	getLogs(): LoggedError[] {
		return [...this.logs];
	}

	/**
	 * Get logs by level
	 */
	getLogsByLevel(level: LoggedError['level']): LoggedError[] {
		return this.logs.filter(log => log.level === level);
	}

	/**
	 * Clear all logs
	 */
	clearLogs(): void {
		this.logs = [];
	}

	/**
	 * Export logs as JSON for debugging
	 */
	exportLogs(): string {
		return JSON.stringify(this.logs, null, 2);
	}

	/**
	 * Get error statistics
	 */
	getStats(): {
		total: number;
		byLevel: Record<LoggedError['level'], number>;
		recent: number; // Last hour
	} {
		const oneHourAgo = new Date(Date.now() - 60 * 60 * 1000);
		
		return {
			total: this.logs.length,
			byLevel: {
				error: this.logs.filter(log => log.level === 'error').length,
				warning: this.logs.filter(log => log.level === 'warning').length,
				info: this.logs.filter(log => log.level === 'info').length,
				debug: this.logs.filter(log => log.level === 'debug').length
			},
			recent: this.logs.filter(log => log.timestamp > oneHourAgo).length
		};
	}
}

// Export singleton instance
export const errorLogger = new ErrorLogger();

// Helper functions for common use cases
export function logApiError(error: any, endpoint: string): void {
	errorLogger.logError(error, {
		component: 'API',
		action: `Request to ${endpoint}`,
	});
}

export function logComponentError(error: any, component: string, action?: string): void {
	errorLogger.logError(error, {
		component,
		action
	});
}

export function logUserAction(action: string, component?: string): void {
	errorLogger.logInfo(`User action: ${action}`, {
		component,
		action
	});
}

// Global error handler setup
if (typeof window !== 'undefined') {
	// Handle unhandled errors
	window.addEventListener('error', (event) => {
		errorLogger.logError(event.error, {
			component: 'Global',
			action: 'Unhandled Error'
		});
	});

	// Handle unhandled promise rejections
	window.addEventListener('unhandledrejection', (event) => {
		errorLogger.logError(event.reason, {
			component: 'Global',
			action: 'Unhandled Promise Rejection'
		});
	});
}