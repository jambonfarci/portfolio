import { describe, it, expect, beforeAll } from 'vitest';
import { apiClient } from '$lib/api/client';
import { projectsStore } from '$lib/stores/projects';
import { skillsStore } from '$lib/stores/skills';
import { profileStore } from '$lib/stores/profile';
import { contactStore } from '$lib/stores/contact';

describe('End-to-End User Workflows', () => {
	beforeAll(async () => {
		// Wait for backend to be ready
		let retries = 5;
		while (retries > 0) {
			try {
				const response = await fetch('http://localhost:3001/health');
				if (response.ok) break;
			} catch (error) {
				console.log('Waiting for backend...');
			}
			retries--;
			await new Promise(resolve => setTimeout(resolve, 1000));
		}
		
		if (retries === 0) {
			throw new Error('Backend not available for testing');
		}
	});

	describe('Portfolio Visitor Workflow', () => {
		it('should complete a typical visitor journey', async () => {
			// 1. Visitor lands on homepage and loads profile
			console.log('Step 1: Loading profile data...');
			await profileStore.loadProfile();
			
			// Profile should be loaded successfully
			const profileResponse = await apiClient.getProfile();
			expect(profileResponse.success).toBe(true);
			expect(profileResponse.data?.name).toBeDefined();
			
			// 2. Visitor views skills overview
			console.log('Step 2: Loading skills data...');
			await skillsStore.loadSkills();
			
			const skillsResponse = await apiClient.getSkills();
			expect(skillsResponse.success).toBe(true);
			expect(Array.isArray(skillsResponse.data)).toBe(true);
			
			// 3. Visitor browses projects
			console.log('Step 3: Loading projects...');
			await projectsStore.loadProjects();
			
			const projectsResponse = await apiClient.getProjects();
			expect(projectsResponse.success).toBe(true);
			expect(Array.isArray(projectsResponse.data)).toBe(true);
			
			// 4. Visitor filters projects by category
			if (projectsResponse.data && projectsResponse.data.length > 0) {
				const firstProject = projectsResponse.data[0];
				console.log(`Step 4: Filtering by category: ${firstProject.category}`);
				
				const filteredResponse = await apiClient.getProjects(firstProject.category);
				expect(filteredResponse.success).toBe(true);
				expect(filteredResponse.data?.every(p => p.category === firstProject.category)).toBe(true);
			}
			
			// 5. Visitor views featured projects
			console.log('Step 5: Loading featured projects...');
			const featuredResponse = await apiClient.getProjects(undefined, true);
			expect(featuredResponse.success).toBe(true);
			expect(featuredResponse.data?.every(p => p.featured === true)).toBe(true);
			
			// 6. Visitor views project details
			if (projectsResponse.data && projectsResponse.data.length > 0) {
				const projectId = projectsResponse.data[0].id;
				console.log(`Step 6: Loading project details for ID: ${projectId}`);
				
				const projectDetailResponse = await apiClient.getProject(projectId);
				expect(projectDetailResponse.success).toBe(true);
				expect(projectDetailResponse.data?.id).toBe(projectId);
			}
			
			// 7. Visitor sends contact message
			console.log('Step 7: Sending contact message...');
			const contactMessage = {
				name: 'Test Visitor',
				email: 'visitor@example.com',
				subject: 'Inquiry about services',
				message: 'Hello! I am interested in your web development services. Could you please provide more information about your availability and rates?'
			};
			
			const contactResult = await contactStore.sendMessage(contactMessage);
			expect(contactResult).toBe(true);
			
			console.log('✅ Complete visitor workflow successful!');
		});
	});

	describe('Data Consistency Workflow', () => {
		it('should maintain data consistency across operations', async () => {
			// 1. Load initial data
			console.log('Loading initial data for consistency check...');
			
			const [profileResponse, projectsResponse, skillsResponse] = await Promise.all([
				apiClient.getProfile(),
				apiClient.getProjects(),
				apiClient.getSkills()
			]);
			
			expect(profileResponse.success).toBe(true);
			expect(projectsResponse.success).toBe(true);
			expect(skillsResponse.success).toBe(true);
			
			// 2. Verify data structure consistency
			console.log('Verifying data structure consistency...');
			
			// Profile should have required fields
			const profile = profileResponse.data!;
			expect(profile.id).toBeDefined();
			expect(profile.name).toBeDefined();
			expect(profile.email).toBeDefined();
			expect(profile.title).toBeDefined();
			expect(profile.bio).toBeDefined();
			expect(profile.location).toBeDefined();
			
			// Projects should have required fields
			const projects = projectsResponse.data!;
			projects.forEach(project => {
				expect(project.id).toBeDefined();
				expect(project.title).toBeDefined();
				expect(project.description).toBeDefined();
				expect(project.category).toBeDefined();
				expect(Array.isArray(project.technologies)).toBe(true);
				expect(typeof project.featured).toBe('boolean');
				expect(project.created_at).toBeDefined();
			});
			
			// Skills should have required fields
			const skills = skillsResponse.data!;
			skills.forEach(skill => {
				expect(skill.id).toBeDefined();
				expect(skill.name).toBeDefined();
				expect(skill.category).toBeDefined();
				expect(typeof skill.level).toBe('number');
				expect(skill.level).toBeGreaterThanOrEqual(1);
				expect(skill.level).toBeLessThanOrEqual(5);
			});
			
			// 3. Verify category consistency
			console.log('Verifying category consistency...');
			
			const projectCategories = [...new Set(projects.map(p => p.category))];
			const skillCategories = [...new Set(skills.map(s => s.category))];
			
			expect(projectCategories.length).toBeGreaterThan(0);
			expect(skillCategories.length).toBeGreaterThan(0);
			
			// Test filtering by each category
			for (const category of projectCategories) {
				const categoryResponse = await apiClient.getProjects(category);
				expect(categoryResponse.success).toBe(true);
				expect(categoryResponse.data?.every(p => p.category === category)).toBe(true);
			}
			
			for (const category of skillCategories) {
				const categoryResponse = await apiClient.getSkills(category);
				expect(categoryResponse.success).toBe(true);
				expect(categoryResponse.data?.every(s => s.category === category)).toBe(true);
			}
			
			console.log('✅ Data consistency verification successful!');
		});
	});

	describe('Error Recovery Workflow', () => {
		it('should handle and recover from various error scenarios', async () => {
			// 1. Test 404 error handling
			console.log('Testing 404 error handling...');
			const notFoundResponse = await apiClient.getProject(99999);
			expect(notFoundResponse.success).toBe(false);
			expect(notFoundResponse.error).toBeDefined();
			
			// 2. Test invalid contact message
			console.log('Testing validation error handling...');
			const invalidMessage = {
				name: '',
				email: 'invalid-email',
				subject: '',
				message: ''
			};
			
			const invalidResult = await contactStore.sendMessage(invalidMessage);
			expect(invalidResult).toBe(false);
			
			// 3. Test recovery with valid data
			console.log('Testing error recovery...');
			const validMessage = {
				name: 'Recovery Test',
				email: 'recovery@example.com',
				subject: 'Recovery Test',
				message: 'This message tests error recovery functionality.'
			};
			
			const recoveryResult = await contactStore.sendMessage(validMessage);
			expect(recoveryResult).toBe(true);
			
			// 4. Verify system is still functional after errors
			console.log('Verifying system functionality after errors...');
			const healthCheck = await apiClient.getProfile();
			expect(healthCheck.success).toBe(true);
			
			console.log('✅ Error recovery workflow successful!');
		});
	});

	describe('Performance and Loading States', () => {
		it('should handle concurrent requests efficiently', async () => {
			console.log('Testing concurrent request handling...');
			
			const startTime = Date.now();
			
			// Make multiple concurrent requests
			const promises = [
				apiClient.getProfile(),
				apiClient.getProjects(),
				apiClient.getSkills(),
				apiClient.getProjects('web'),
				apiClient.getSkills('Frontend')
			];
			
			const results = await Promise.all(promises);
			
			const endTime = Date.now();
			const duration = endTime - startTime;
			
			// All requests should succeed
			results.forEach(result => {
				expect(result.success).toBe(true);
			});
			
			// Should complete in reasonable time (less than 5 seconds)
			expect(duration).toBeLessThan(5000);
			
			console.log(`✅ Concurrent requests completed in ${duration}ms`);
		});

		it('should handle large data sets efficiently', async () => {
			console.log('Testing large data set handling...');
			
			const startTime = Date.now();
			
			// Load all data
			const [projects, skills] = await Promise.all([
				apiClient.getProjects(),
				apiClient.getSkills()
			]);
			
			const endTime = Date.now();
			const duration = endTime - startTime;
			
			expect(projects.success).toBe(true);
			expect(skills.success).toBe(true);
			
			// Should handle data efficiently
			expect(duration).toBeLessThan(2000);
			
			console.log(`✅ Large data sets loaded in ${duration}ms`);
			console.log(`   - Projects: ${projects.data?.length || 0}`);
			console.log(`   - Skills: ${skills.data?.length || 0}`);
		});
	});
});