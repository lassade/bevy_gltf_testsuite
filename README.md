# Bevy glTF Test Suite

Tries to load all glTF samples from the KhronosGroup/glTF-Sample-Models repository

## Methodology

1. Each model is loaded one at the time
2. Load will timeout at 30s
3. `run.sh` is used to re-run the bench until it finishes testing everything

# Results

|Model|Screenshot|Load|Spawn|Glitch|
|-----|----------|----|-----|------|
|[2 Cylinder Engine](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/2CylinderEngine)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/2CylinderEngine/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Alpha Blend Mode Test](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AlphaBlendModeTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AlphaBlendModeTest/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Animated Cube](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AnimatedCube)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AnimatedCube/screenshot/screenshot.gif)|:heavy_check_mark:| | |
|[Animated Morph Cube](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AnimatedMorphCube)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AnimatedMorphCube/screenshot/screenshot.gif)|:x:| | |
|[Animated Morph Sphere](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AnimatedMorphSphere)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AnimatedMorphSphere/screenshot/screenshot.gif)|:x:| | |
|[Animated Triangle](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AnimatedTriangle)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AnimatedTriangle/screenshot/screenshot.gif)|:x:| | |
|[Antique Camera](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AntiqueCamera)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AntiqueCamera/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Attenuation Test](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AttenuationTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/AttenuationTest/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Avocado](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Avocado)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Avocado/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Barramundi Fish](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BarramundiFish)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BarramundiFish/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Boom Box](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoomBox)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoomBox/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Boom Box With Axes](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoomBoxWithAxes)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoomBoxWithAxes/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Box](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Box)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Box/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Box  With  Spaces](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Box With Spaces)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Box With Spaces/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Box Animated](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoxAnimated)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoxAnimated/screenshot/screenshot.gif)|:x:| | |
|[Box Interleaved](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoxInterleaved)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoxInterleaved/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Box Textured](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoxTextured)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoxTextured/screenshot/screenshot.png)|:hourglass:| | |
|[Box Textured Non Power Of Two](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoxTexturedNonPowerOfTwo)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoxTexturedNonPowerOfTwo/screenshot/screenshot.png)|:hourglass:| | |
|[Box Vertex Colors](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoxVertexColors)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BoxVertexColors/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Brain Stem](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BrainStem)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/BrainStem/screenshot/screenshot.gif)|:x:| | |
|[Buggy](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Buggy)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Buggy/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Cameras](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Cameras)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Cameras/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Cesium Man](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/CesiumMan)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/CesiumMan/screenshot/screenshot.gif)|:hourglass:| | |
|[Cesium Milk Truck](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/CesiumMilkTruck)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/CesiumMilkTruck/screenshot/screenshot.gif)|:heavy_check_mark:| | |
|[Clear Coat Test](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/ClearCoatTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/ClearCoatTest/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Corset](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Corset)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Corset/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Cube](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Cube)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Cube/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Damaged Helmet](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/DamagedHelmet)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/DamagedHelmet/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Duck](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Duck)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Duck/screenshot/screenshot.png)|:hourglass:| | |
|[Environment Test](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/EnvironmentTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/EnvironmentTest/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Flight Helmet](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/FlightHelmet)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/FlightHelmet/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Fox](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Fox)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Fox/screenshot/screenshot.jpg)|:hourglass:| | |
|[Gearbox Assy](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/GearboxAssy)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/GearboxAssy/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Interpolation Test](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/InterpolationTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/InterpolationTest/screenshot/screenshot.gif)|:x:| | |
|[Lantern](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Lantern)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/Lantern/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Materials Variants Shoe](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MaterialsVariantsShoe)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MaterialsVariantsShoe/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Metal Rough Spheres](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MetalRoughSpheres)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MetalRoughSpheres/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Metal Rough Spheres No Textures](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MetalRoughSpheresNoTextures)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MetalRoughSpheresNoTextures/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Morph Primitives Test](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MorphPrimitivesTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MorphPrimitivesTest/screenshot/screenshot.jpg)|:hourglass:| | |
|[Morph Stress Test](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MorphStressTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MorphStressTest/screenshot/screenshot.jpg)|:x:| | |
|[Multi UVTest](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MultiUVTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/MultiUVTest/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Normal Tangent Mirror Test](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/NormalTangentMirrorTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/NormalTangentMirrorTest/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Normal Tangent Test](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/NormalTangentTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/NormalTangentTest/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Orientation Test](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/OrientationTest)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/OrientationTest/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Reciprocating Saw](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/ReciprocatingSaw)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/ReciprocatingSaw/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Recursive Skeletons](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/RecursiveSkeletons)|![](https://github.com/KhronosGroup/glTF-Sample-Models/blob/master/2.0/RecursiveSkeletons/screenshot/screenshot.jpg)|:x:| | |
