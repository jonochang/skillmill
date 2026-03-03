# Product Brief: SkillMill

**Tagline:** A programmatic, constraint-driven worksheet factory for any skill-based curriculum.

---

## 1. Executive Summary

**SkillMill** is an automated, high-fidelity educational content generator. Its purpose is to programmatically output highly structured, pedagogically sound practice worksheets across any discipline that benefits from systematic, Kumon-style drilling — Mathematics, French, English grammar and vocabulary, Music theory, and beyond.

SkillMill is discipline-agnostic by design. A **Curriculum Graph** defines the skills and their dependencies for any subject; **Schemas** encode the generative rules for each skill; and a shared **Rendering Pipeline** produces consistent, beautifully typeset student worksheets and answer keys. The first reference implementation targets the Singapore MOE Mathematics syllabus (Primary 1 through O-Level), which serves as the proving ground for the architecture. All other disciplines plug into the same engine.

Unlike static question banks, SkillMill uses constraint-driven schema generation and spiral progression policies to produce infinite, non-repeating practice material calibrated precisely to a student's level — the same principle that makes Kumon effective, but applied to any subject.

---

## 2. Cross-Functional Focus

### Product Manager — Curriculum & Rules

Translate syllabi and pedagogical frameworks into system rules.

- **Source of Truth:** Map syllabi (e.g., MOE Mathematics, DELF French, Cambridge English) into structured curriculum graphs defining learning outcomes, prerequisites, and progression.
- **Worksheet Composition Policies:** Define the pedagogical mix per discipline (e.g., a P4 fractions worksheet: 70% target skill, 20% P3 review, 10% non-routine word problems; a French B1 worksheet: 60% target grammar, 25% vocabulary in context, 15% reading comprehension).
- **Progression & Variation:** Define Kumon-style variation steps (horizontal/vertical variation and near-miss distractors) appropriate to each discipline.
- **Discipline Onboarding:** Establish the intake process for adding a new subject — defining its node schema, difficulty axes, and allowed item types.

### Designer — Output & Pedagogy

Visual translation of concepts across disciplines and the generator UI.

- **Typesetting & Layout:** Design for print/PDF. Create Typst/LaTeX templates that look like premium, professionally published worksheets — with layouts appropriate per discipline (math working space, fill-in-the-blank prose, conjugation tables, etc.).
- **Visualising Pedagogy:** For Mathematics, design the SVG/TikZ systems for Bar Models and geometry figures. For other disciplines, design equivalent visual scaffolds (e.g., sentence-structure diagrams for grammar, timeline visuals for verb tenses).
- **User Interface:** Design the web dashboard where teachers and tutors select a discipline, generate custom batches, set student profiles, and view mastery data.

### Engineer — Engine & Pipeline

Build a deterministic, discipline-agnostic generation pipeline.

- **Constraint-Driven Generation:** Build the core engine (Rust or Go) with a plugin model: each discipline registers its own schema types and constraint validators. Shared infrastructure handles variable sampling, worksheet composition, and rendering dispatch.
- **Symbolic Backend:** Integrate a Computer Algebra System (e.g. Symbolica) for mathematical disciplines requiring algebraic validation. Design the backend interface so that discipline-specific validators (a grammar checker, a conjugation verifier) can be swapped in for non-math subjects.
- **Rendering & CI/CD:** Build the pipeline that turns a JSON worksheet spec into a compiled PDF in under 3 seconds. The test suite must guarantee 0% errors in answer keys across all registered disciplines.

---

## 3. System Architecture

### A. Curriculum Graph

The foundation is a machine-readable graph (YAML/JSON) of a syllabus. Any discipline can be registered. Every concept is a node containing:

- Learning outcomes and prerequisites
- Allowed item types and representations (e.g., drill, fill-in-the-blank, multiple choice, short answer, diagram)
- Difficulty axes (appropriate to the discipline: number range for maths, tense complexity for French, etc.)

### B. Schema Engine

SkillMill does not store static questions — it stores **schemas**: code blocks that define variables, constraints, and rendering instructions for a skill node. The engine dynamically generates the question, correct answer, and worked solution from each schema. Schemas are discipline-specific; the engine that executes them is shared.

### C. Semantic Item & Visual Generator

For structured disciplines (e.g., Mathematics word problems, French sentence transformation), the engine generates items from an intermediate semantic spec before rendering:

1. Engine produces a structured item spec (e.g., a bar-model spec for a maths ratio problem, or a sentence template spec for a French grammar drill).
2. Spec is rendered into the appropriate visual or textual format alongside the item.

### D. Automated Layout & Rendering

1. Engine outputs a JSON worksheet spec based on the composition policy for the selected discipline and level.
2. Spec is injected into the Typst or LaTeX rendering pipeline.
3. Pipeline outputs two PDFs: a Student Worksheet and a Teacher Answer Key.

---

## 4. Technology Stack

| Layer | Technology |
|---|---|
| Data | YAML / JSON (Curriculum Graphs & Policies) |
| Core Engine | Rust or Go |
| Discipline Validators | Plugin interface; Symbolica (Rust) CAS for maths; extensible for other disciplines |
| Rendering | Typst (preferred) or LaTeX |
| CI/CD | Property tests, equivalence/validation tests per discipline, PDF visual snapshot tests |

---

## 5. Implementation Roadmap

### Phase 1 — Foundation: Singapore Mathematics P1–P3

Establish the core architecture using the Singapore Math syllabus as the reference implementation.

- **PM:** Digitise curriculum graph for P1–P3 Number & Algebra.
- **Engineering:** Build basic arithmetic drill schemas and simple word problems; establish the Typst/LaTeX PDF pipeline; define the discipline plugin interface.
- **Design:** Create base typography and layout templates for PDFs.

### Phase 2 — Full Primary Mathematics & Bar Modelling (P4–P6)

- **PM:** Expand curriculum graph to Measurement, Geometry, and Statistics.
- **Engineering:** Implement the semantic word-problem engine and automated SVG/TikZ bar-model generation.
- **Design:** Standardise the visual design system for Bar Models and geometry shapes.

### Phase 3 — Secondary Mathematics Core (G1–G3)

- **Engineering:** Integrate CAS backend (Symbolica) for algebraic validation; build schemas for secondary algebra, geometry, and statistics.

### Phase 4 — Advanced Secondary Mathematics (O-Level / Additional Mathematics)

- **PM:** Map SEAB syllabus and exam structures.
- **Engineering:** Advanced calculus schemas; automated generation of multi-step marking schemes.
- **Design:** Update templates to match O-Level exam paper formatting.

### Phase 5 — Second Discipline: French or English

Validate that the architecture generalises by onboarding a non-mathematics discipline.

- **PM:** Define the curriculum graph and composition policies for the target discipline (e.g., DELF French levels A1–B2, or Cambridge English grammar & vocabulary).
- **Engineering:** Implement the first non-math schema types (conjugation drills, fill-in-the-blank, vocabulary matching); integrate or build an appropriate validator.
- **Design:** Design discipline-appropriate worksheet layouts and visual scaffolds.

### Phase 6 — Web App & Adaptive Mastery

- **Design:** Design the SaaS web interface for tutors and teachers, supporting multi-discipline selection.
- **Engineering:** Build user database, mastery tracking per discipline, and the adaptive algorithm for daily practice sets.

---

## 6. Definition of Success

| Metric | Target |
|---|---|
| Architecture | Core engine supports registration of any discipline without changes to shared infrastructure |
| Curriculum coverage | 100% of targeted syllabus nodes mapped to at least one functional, generative schema |
| Quality guarantee | 0% mathematical or typesetting errors in generated answer keys (enforced via CI/CD) |
| Performance | Generate a 20-page worksheet + step-by-step answer key PDF in under 3 seconds |
