from __future__ import annotations

import os

from agentic_crew import discover_packages, get_crew_config, run_crew_auto


def main():
    """
    Minimal example of running a crew programmatically.
    
    This example assumes you have a package with a crew configuration.
    In this case, we'll try to use the 'vendor-connectors' example package
    if it's discoverable.
    """
    # 1. Discover all packages with .crew/ (or .crewai/, etc.) directories
    # By default, it looks in the 'packages/' directory of the workspace root.
    packages = discover_packages()
    
    if not packages:
        print("No crew packages found. Try running from the workspace root.")
        return

    print(f"Discovered packages: {list(packages.keys())}")
    
    # 2. Pick a package and a crew
    # For this example, we'll just take the first one found
    package_name = list(packages.keys())[0]
    package_path = packages[package_name]
    
    # Let's list crews in this package
    from agentic_crew import list_crews
    package_crews = list_crews(package_name)
    
    if not package_crews.get(package_name):
        print(f"No crews found in package {package_name}")
        return
        
    crew_name = package_crews[package_name][0]["name"]
    print(f"Running crew: {package_name}/{crew_name}")
    
    # 3. Load the crew configuration
    config = get_crew_config(package_path, crew_name)
    
    # 4. Run the crew
    # The framework will be auto-detected based on what's installed
    # (CrewAI > LangGraph > Strands)
    inputs = {"input": "Create a simple HTTP connector for a weather API"}
    
    try:
        # Note: This requires ANTHROPIC_API_KEY to be set if using default Claude models
        if not os.environ.get("ANTHROPIC_API_KEY"):
            print("Warning: ANTHROPIC_API_KEY not set. Execution might fail.")
            
        result = run_crew_auto(config, inputs=inputs)
        
        print("\n--- Result ---")
        print(result)
        
    except Exception as e:
        print(f"Error running crew: {e}")

if __name__ == "__main__":
    main()
