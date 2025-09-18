import { describe, it, expect, beforeEach, vi } from 'vitest';
import { errorLogger, logApiError, logComponentError, logUserAction } from '$lib/utils/errorLogger';

describe('Error Logger', () => {
	beforeEach(() => {
		errorLogger.clearLogs();
		vi.clearAllMocks();
	});

	describe('Error Filtering', () => {
		it('should ignore IDE-related errors', () => {
			const ideErrors = [
				new Error('Failed to initialize messaging: tx_attempts_exceeded'),
				new Error('tx_ack_timeout'),
				new Error('host-additional-hooks error'),
				new Error('getSingleton failed'),
				new Error('kiro-ide connection error')
			];

			ideErrors.forEach(error => {
				errorLogger.logError(error);
			});

			const logs = errorLogger.getLogs();
			expect(logs).toHaveLength(0);
		});

		it('should log application errors', () => {
			const appErrors = [
				new Error('API request failed'),
				new Error('Component render error'),
				new Error('User validation failed')
			];

			appErrors.forEach(error => {
				errorLogger.logError(error);
			});

			const logs = errorLogger.getLogs();
			expect(logs).toHaveLength(3);
			expect(logs.every(log => log.level === 'error')).toBe(true);
		});
	});

	describe('Logging Functions', () => {
		it('should log errors with context', () => {
			const error = new Error('Test error');
			const context = {
				component: 'TestComponent',
				action: 'testAction'
			};

			errorLogger.logError(error, context);

			const logs = errorLogger.getLogs();
			expect(logs).toHaveLength(1);
			expect(logs[0].message).toBe('Test error');
			expect(logs[0].context?.component).toBe('TestComponent');
			expect(logs[0].context?.action).toBe('testAction');
		});

		it('should log warnings', () => {
			errorLogger.logWarning('Test warning', { component: 'TestComponent' });

			const logs = errorLogger.getLogs();
			expect(logs).toHaveLength(1);
			expect(logs[0].level).toBe('warning');
			expect(logs[0].message).toBe('Test warning');
		});

		it('should log info messages', () => {
			errorLogger.logInfo('Test info', { component: 'TestComponent' });

			const logs = errorLogger.getLogs();
			expect(logs).toHaveLength(1);
			expect(logs[0].level).toBe('info');
			expect(logs[0].message).toBe('Test info');
		});
	});

	describe('Helper Functions', () => {
		it('should log API errors with correct context', () => {
			const error = new Error('API Error');
			logApiError(error, '/api/test');

			const logs = errorLogger.getLogs();
			expect(logs).toHaveLength(1);
			expect(logs[0].context?.component).toBe('API');
			expect(logs[0].context?.action).toBe('Request to /api/test');
		});

		it('should log component errors with correct context', () => {
			const error = new Error('Component Error');
			logComponentError(error, 'TestComponent', 'render');

			const logs = errorLogger.getLogs();
			expect(logs).toHaveLength(1);
			expect(logs[0].context?.component).toBe('TestComponent');
			expect(logs[0].context?.action).toBe('render');
		});

		it('should log user actions', () => {
			logUserAction('button_click', 'ContactForm');

			const logs = errorLogger.getLogs();
			expect(logs).toHaveLength(1);
			expect(logs[0].level).toBe('info');
			expect(logs[0].message).toBe('User action: button_click');
			expect(logs[0].context?.component).toBe('ContactForm');
		});
	});

	describe('Log Management', () => {
		it('should limit the number of logs', () => {
			// Create more logs than the limit (100)
			for (let i = 0; i < 150; i++) {
				errorLogger.logError(new Error(`Error ${i}`));
			}

			const logs = errorLogger.getLogs();
			expect(logs.length).toBeLessThanOrEqual(100);
		});

		it('should get logs by level', () => {
			errorLogger.logError(new Error('Error 1'));
			errorLogger.logError(new Error('Error 2'));
			errorLogger.logWarning('Warning 1');
			errorLogger.logInfo('Info 1');

			const errorLogs = errorLogger.getLogsByLevel('error');
			const warningLogs = errorLogger.getLogsByLevel('warning');
			const infoLogs = errorLogger.getLogsByLevel('info');

			expect(errorLogs).toHaveLength(2);
			expect(warningLogs).toHaveLength(1);
			expect(infoLogs).toHaveLength(1);
		});

		it('should provide correct statistics', () => {
			errorLogger.logError(new Error('Error 1'));
			errorLogger.logError(new Error('Error 2'));
			errorLogger.logWarning('Warning 1');
			errorLogger.logInfo('Info 1');

			const stats = errorLogger.getStats();
			expect(stats.total).toBe(4);
			expect(stats.byLevel.error).toBe(2);
			expect(stats.byLevel.warning).toBe(1);
			expect(stats.byLevel.info).toBe(1);
			expect(stats.byLevel.debug).toBe(0);
		});

		it('should export logs as JSON', () => {
			errorLogger.logError(new Error('Test error'));
			errorLogger.logWarning('Test warning');

			const exported = errorLogger.exportLogs();
			const parsed = JSON.parse(exported);

			expect(Array.isArray(parsed)).toBe(true);
			expect(parsed).toHaveLength(2);
			expect(parsed[0].message).toBe('Test error');
			expect(parsed[1].message).toBe('Test warning');
		});

		it('should clear all logs', () => {
			errorLogger.logError(new Error('Error 1'));
			errorLogger.logWarning('Warning 1');
			
			expect(errorLogger.getLogs()).toHaveLength(2);
			
			errorLogger.clearLogs();
			
			expect(errorLogger.getLogs()).toHaveLength(0);
		});
	});

	describe('Context Enhancement', () => {
		it('should add timestamp and URL to context', () => {
			const beforeTime = new Date();
			errorLogger.logError(new Error('Test error'));
			const afterTime = new Date();

			const logs = errorLogger.getLogs();
			expect(logs).toHaveLength(1);
			
			const log = logs[0];
			expect(log.context?.timestamp).toBeDefined();
			expect(log.context?.url).toBeDefined();
			
			const logTime = new Date(log.context!.timestamp!);
			expect(logTime.getTime()).toBeGreaterThanOrEqual(beforeTime.getTime());
			expect(logTime.getTime()).toBeLessThanOrEqual(afterTime.getTime());
		});

		it('should include user agent in context', () => {
			errorLogger.logError(new Error('Test error'));

			const logs = errorLogger.getLogs();
			expect(logs[0].context?.userAgent).toBeDefined();
		});
	});
});