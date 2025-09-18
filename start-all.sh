#!/bin/bash

echo "🚀 Démarrage du portfolio..."

# Arrêter tous les processus existants
pkill -f "vite\|cargo.*portfolio\|node.*vite" 2>/dev/null || true

# Attendre un peu
sleep 2

# Démarrer le backend
echo "📡 Démarrage du backend..."
cd backend
export DATABASE_URL=sqlite:../data/portfolio.db
cargo run &
BACKEND_PID=$!
cd ..

# Attendre que le backend démarre
echo "⏳ Attente du démarrage du backend..."
sleep 10

# Vérifier que le backend fonctionne
if curl -s http://localhost:3001/health > /dev/null; then
    echo "✅ Backend démarré avec succès"
else
    echo "❌ Erreur: Le backend ne répond pas"
    exit 1
fi

# Démarrer le frontend
echo "🎨 Démarrage du frontend..."
cd frontend
npm run dev &
FRONTEND_PID=$!
cd ..

# Attendre que le frontend démarre
echo "⏳ Attente du démarrage du frontend..."
sleep 15

# Vérifier que le frontend fonctionne
if curl -s http://localhost:5173/ > /dev/null; then
    echo "✅ Frontend démarré avec succès"
else
    echo "❌ Erreur: Le frontend ne répond pas"
fi

echo ""
echo "🎉 Portfolio démarré !"
echo "📱 Frontend: http://localhost:5173"
echo "🔧 Backend API: http://localhost:3001"
echo ""
echo "Pour arrêter les services, utilisez: pkill -f 'vite|cargo.*portfolio'"