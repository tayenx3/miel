const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const module = b.createModule(.{
        .root_source_file = null,
        .target = target,
        .optimize = optimize,
        .link_libc = true
    });
    
    const exe = b.addExecutable(.{
        .name = "ivo",
        .root_module = module
    });
    
    module.addCSourceFiles(.{
        .files = &.{
            "src/main.c",
        },
        .flags = &.{
            "-Wall",
            "-Wextra",
            "-std=c23",
            "-g",
        },
    });
    
    module.addIncludePath(b.path("src"));
    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    const run_step = b.step("run", "Run the Ivo compiler");
    run_step.dependOn(&run_cmd.step);
}
