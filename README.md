# Dyson: Your Self-Hosted ML Model Interface

> As local models become more powerful, they should also get easier to use. Dyson does just that.

## Overview

Dyson is a platform for deploying and managing machine learning models locally on your own server. Powered by Docker, Dyson automatically generates an interface & API to interact with any machine learning model.

- 🔌 **Plug-and-Play**: Drop in a model's name, and Dyson sets up the rest.
- 📈 **Built for Scale**: Built on Kubernetes, scaling is not an afterthought.
- 🔑 **Fully Local**: Run everything on your own hardware. Your data, your rules.
- 🌐 **Model Discovery**: Pull models directly from the [Replicate](https://replicate.com) registry or any custom registry.
- ✨ **Auto-Generated Interface**: Dyson creates a custom UI for each model, abstracting away the technicalities.

## Architecture

- **API** (Rust): Retrieves data from postgres and manages predictions.
- **Frontend** (Next.js): User interface for interacting with models and seeing past results. Consumes the public API.
- **Director** (Rust): Interacts and communicates with Cog-based Docker containers. Updates API backend on model status and results.
- **Database** (PostgreSQL): Persistent storage for predictions, user data, etc.
- **Redis**: Keeps track of queued predictions. Used by the API backend to communicate with the Director.
- **Kubernetes**: Orchestration of all services, ensuring scalability and high availability. Manages deployment and scaling of all components.
  
## Installation

We currently provide a guide to [set up Dyson on a brand new Arch Linux machine](/infra/README.md). A guide for running it alongside your existing OS will be provided soon-ish!

## Development

Dyson uses [Tilt](https://tilt.dev) to emulate a Kubernetes cluster when developing locally. To run Dyson on development, [install Tilt](https://docs.tilt.dev/install.html), then run `tilt up`. This should bring up all related services, and start watching for changes.

## License
This project is licensed under the MIT License - see the [LICENSE file](LICENSE) for details.
