# Portfolio Deployment Guide

This guide covers deploying the portfolio application in production environments.

## Prerequisites

- Docker and Docker Compose installed
- At least 2GB RAM and 10GB disk space
- Domain name configured (for production)
- SSL certificate (for HTTPS)

## Quick Start

### Local Production Build

```bash
# Clone the repository
git clone <repository-url>
cd portfolio

# Deploy using the deployment script
./deploy.sh

# The application will be available at:
# - Frontend: http://localhost
# - Backend API: http://localhost:3001
```

## Manual Deployment

### 1. Environment Configuration

Copy and customize the environment files:

```bash
# Backend configuration
cp backend/.env.production backend/.env
# Edit backend/.env with your production values

# Frontend configuration  
cp frontend/.env.production frontend/.env
# Edit frontend/.env with your production values
```

### 2. Build and Deploy

```bash
# Build and start all services
docker-compose -f docker-compose.prod.yml up --build -d

# Check service status
docker-compose -f docker-compose.prod.yml ps

# View logs
docker-compose -f docker-compose.prod.yml logs -f
```

### 3. Verify Deployment

```bash
# Run smoke tests
./scripts/smoke-tests.sh

# Check individual services
curl http://localhost/health          # Frontend health
curl http://localhost:3001/api/projects  # Backend API
```

## Production Configuration

### Backend Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Log level | `info` |
| `DATABASE_URL` | SQLite database path | `sqlite:///app/data/portfolio.db` |
| `HOST` | Server bind address | `0.0.0.0` |
| `PORT` | Server port | `3001` |
| `CORS_ALLOWED_ORIGINS` | Allowed CORS origins | `*` |
| `JWT_SECRET` | JWT signing secret | Required for auth |

### Frontend Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `VITE_API_BASE_URL` | Backend API URL | `/api` |
| `VITE_APP_NAME` | Application name | `Portfolio` |
| `VITE_APP_VERSION` | Application version | `1.0.0` |

## SSL/HTTPS Setup

### Using Let's Encrypt with Certbot

1. Install Certbot:
```bash
sudo apt-get update
sudo apt-get install certbot python3-certbot-nginx
```

2. Obtain SSL certificate:
```bash
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com
```

3. Update nginx configuration to include SSL settings.

### Using Custom SSL Certificate

1. Place your certificate files in `frontend/ssl/`:
   - `cert.pem` - SSL certificate
   - `key.pem` - Private key

2. Update `frontend/nginx.conf` to include SSL configuration:
```nginx
server {
    listen 443 ssl http2;
    ssl_certificate /etc/ssl/certs/cert.pem;
    ssl_certificate_key /etc/ssl/private/key.pem;
    # ... rest of configuration
}
```

## Database Management

### Backup

```bash
# Create database backup
docker-compose -f docker-compose.prod.yml exec backend \
    sqlite3 /app/data/portfolio.db ".backup /app/data/backup_$(date +%Y%m%d).db"

# Copy backup to host
docker cp portfolio-backend-prod:/app/data/backup_$(date +%Y%m%d).db ./
```

### Restore

```bash
# Stop services
docker-compose -f docker-compose.prod.yml down

# Replace database file
docker-compose -f docker-compose.prod.yml run --rm backend \
    cp /app/data/backup_YYYYMMDD.db /app/data/portfolio.db

# Start services
docker-compose -f docker-compose.prod.yml up -d
```

## Monitoring and Logging

### View Logs

```bash
# All services
docker-compose -f docker-compose.prod.yml logs -f

# Specific service
docker-compose -f docker-compose.prod.yml logs -f backend
docker-compose -f docker-compose.prod.yml logs -f frontend
```

### Health Checks

The deployment includes health checks for both services:

- Frontend: `http://localhost/health`
- Backend: `http://localhost:3001/health` (internal)

### Resource Monitoring

```bash
# Container resource usage
docker stats

# Disk usage
docker system df

# Clean up unused resources
docker system prune -a
```

## Scaling and Performance

### Horizontal Scaling

To run multiple backend instances:

```yaml
# In docker-compose.prod.yml
services:
  backend:
    # ... existing configuration
    deploy:
      replicas: 3
```

### Performance Tuning

1. **Database Optimization**:
   - Enable WAL mode for SQLite
   - Regular VACUUM operations
   - Index optimization

2. **Nginx Optimization**:
   - Enable gzip compression (already configured)
   - Optimize worker processes
   - Configure caching headers

3. **Application Optimization**:
   - Enable Rust release optimizations
   - Configure connection pooling
   - Implement request rate limiting

## Troubleshooting

### Common Issues

1. **Port conflicts**:
   ```bash
   # Check what's using port 80/443
   sudo netstat -tlnp | grep :80
   sudo netstat -tlnp | grep :443
   ```

2. **Permission issues**:
   ```bash
   # Fix data directory permissions
   sudo chown -R 1000:1000 data/
   ```

3. **Database locked**:
   ```bash
   # Stop all services and restart
   docker-compose -f docker-compose.prod.yml down
   docker-compose -f docker-compose.prod.yml up -d
   ```

### Debug Mode

Enable debug logging:

```bash
# Set environment variable
export RUST_LOG=debug

# Restart services
docker-compose -f docker-compose.prod.yml restart backend
```

## Security Considerations

1. **Environment Variables**: Never commit production secrets to version control
2. **Database**: Ensure database files have proper permissions
3. **Network**: Use Docker networks to isolate services
4. **Updates**: Regularly update base images and dependencies
5. **Monitoring**: Implement log monitoring and alerting

## Backup Strategy

1. **Automated Backups**:
   ```bash
   # Add to crontab for daily backups
   0 2 * * * /path/to/backup-script.sh
   ```

2. **Backup Retention**: Keep backups for at least 30 days

3. **Off-site Storage**: Store backups in cloud storage or remote location

## Updates and Maintenance

### Application Updates

```bash
# Pull latest code
git pull origin main

# Rebuild and deploy
./deploy.sh

# Verify deployment
./scripts/smoke-tests.sh
```

### System Updates

```bash
# Update base images
docker-compose -f docker-compose.prod.yml pull

# Restart with new images
docker-compose -f docker-compose.prod.yml up -d
```

## Support

For deployment issues:

1. Check the logs: `docker-compose -f docker-compose.prod.yml logs`
2. Run smoke tests: `./scripts/smoke-tests.sh`
3. Verify configuration files
4. Check system resources and permissions

## Performance Benchmarks

Expected performance metrics:

- **Response Time**: < 100ms for API endpoints
- **Throughput**: > 1000 requests/second
- **Memory Usage**: < 512MB per service
- **Startup Time**: < 30 seconds

Monitor these metrics and adjust resources as needed.