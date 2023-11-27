from conans import ConanFile, CMake
from os import path
from glob import glob

class SampleProject(ConanFile):
    settings = "os", "arch", "compiler", "build_type"
    requires = []
    options = {"shared": [True, False], "fPIC": [True, False]}
    default_options = {"shared": False, "fPIC": True}

    def build(self):
        cmake = CMake(self)
        cmake.definitions["CMAKE_EXPORT_COMPILE_COMMANDS"] = "ON"
        if self.should_configure:
            cmake.configure()
        if self.should_build:
            cmake.build()

    def package(self):
        self.copy("*", dst="lib", src=path.join("build", "lib"))
        self.copy("*", dst="include", src="include")
        for include_path in glob("../src/**/include", recursive=False):
            dst_include_path = "./include/"
            print("Copying %s to %s" % (include_path, dst_include_path))
            self.copy("*", dst=dst_include_path, src=include_path)
