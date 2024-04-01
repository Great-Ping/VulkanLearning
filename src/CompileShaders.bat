@echo on
set glslc=C:\Program Files\VulkanSDK\Bin\glslc.exe
set shadersDir=%cd%\Assets\Shaders

if %outDir%=="" set outDir=%shadersDir%\Compiled
if not exist %outDir% md %outDir%

set getFiles=dir /b /a:-D "%shadersDir%"

for /f %%f in ('%getFiles%') do "%glslc%" "%shadersDir%\%%f" -o "%outDir%\%%f.spv" 

@echo on
@echo shader assembly is complete