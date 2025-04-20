from ghidra.program.util import ProgramLocation
from ghidra.util.task import ConsoleTaskMonitor

def analyze_solana_program():
    monitor = ConsoleTaskMonitor()
    current_program = getCurrentProgram()
    listing = current_program.getListing()
    
    print("=== Solana Program Inspector ===")
    print(f"Analyzing: {current_program.name}")
    
    # Detect SBF magic number
    magic = b"\x7fSOF"
    for block in current_program.getMemory().getBlocks():
        data = block.getData().readBytes(0, 4)
        if data == magic:
            print(f"Found SBF program at {block.name}")
    
    print("Analysis complete")