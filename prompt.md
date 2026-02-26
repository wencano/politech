# Politech Master Specification: Geospatial-Behavioral War Room

Act as a Senior Full-Stack Engineer and Data Architect. We are building "Politech," a geospatial simulation platform for elections and policy stress-testing in the Philippines.

## Tech Stack
- Frontend: Next.js 15 (App Router), Tailwind CSS, Shadcn UI
- Maps: Mapbox GL JS (with Mapbox-Draw for custom boundary selection)
- State Management: TanStack Query (Data fetching) & Zustand (Map/Filter states)
- Analytics: Math.js for Tensor/Matrix operations in the browser

## Core Logic: The "Alignment Engine"
The app simulates the "resonance" between a Policy/Message and a Population.
1. Message Vector: Convert user input (policy text) into a 12-dimension characteristic tensor (Price, Risk, Social Value, etc.)
2. Population Tensor: Each Barangay polygon contains a normalized demographic tensor (Age, Income, Education, Density).
3. Alignment Calculation: Perform a Dot Product [Message] • [Barangay] to calculate a "Resonance Score" (-1 to 1).

## Phase 1 Instructions: The "Battle Map" UI
Build the primary dashboard with the following components:
1. Full-screen Mapbox Map: Render 42,000+ Barangay polygons via GeoJSON. Use a 'fill' layer for the Resonance Score heatmap (Red-Yellow-Green).
2. "Policy Input" Sidebar: A text area to input a policy/message. When 'Simulate' is clicked, generate a mock characteristic tensor and update the Map's color scale.
3. Demographic Filter: Range sliders for Age and Income that update the Map colors in real-time based on your Tensor math.

## Code Quality Rules
- Type Safety: Everything must be strictly typed in TypeScript.
- Performance: Use 'Mapbox Expression' syntax for heatmap coloring to avoid expensive React re-renders of 40k+ polygons.
- Component Logic: Keep UI components in @/components/ui and simulation logic in @/lib/engine.ts.

Let's start by initializing the Mapbox component and the GeoJSON layer for the Philippines.