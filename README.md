# Bevy glTF Test Suite

Tries to load all glTF samples from the KhronosGroup/glTF-Sample-Models repository

## Methodology

1. Each model is loaded one at the time
2. Load will timeout at 30s
3. `run.sh` is used to re-run the bench until it finishes testing everything

# Results

|Model|Screenshot|Load|Spawn|Glitch|
|-----|----------|----|-----|------|
|[2 Cylinder Engine](assets/gltf_samples/2.0/2CylinderEngine)|![](assets/gltf_samples/2.0/2CylinderEngine/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Alpha Blend Mode Test](assets/gltf_samples/2.0/AlphaBlendModeTest)|![](assets/gltf_samples/2.0/AlphaBlendModeTest/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Animated Cube](assets/gltf_samples/2.0/AnimatedCube)|![](assets/gltf_samples/2.0/AnimatedCube/screenshot/screenshot.gif)|:heavy_check_mark:| | |
|[Animated Morph Cube](assets/gltf_samples/2.0/AnimatedMorphCube)|![](assets/gltf_samples/2.0/AnimatedMorphCube/screenshot/screenshot.gif)|:x:| | |
|[Animated Morph Sphere](assets/gltf_samples/2.0/AnimatedMorphSphere)|![](assets/gltf_samples/2.0/AnimatedMorphSphere/screenshot/screenshot.gif)|:x:| | |
|[Animated Triangle](assets/gltf_samples/2.0/AnimatedTriangle)|![](assets/gltf_samples/2.0/AnimatedTriangle/screenshot/screenshot.gif)|:x:| | |
|[Antique Camera](assets/gltf_samples/2.0/AntiqueCamera)|![](assets/gltf_samples/2.0/AntiqueCamera/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Attenuation Test](assets/gltf_samples/2.0/AttenuationTest)|![](assets/gltf_samples/2.0/AttenuationTest/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Avocado](assets/gltf_samples/2.0/Avocado)|![](assets/gltf_samples/2.0/Avocado/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Barramundi Fish](assets/gltf_samples/2.0/BarramundiFish)|![](assets/gltf_samples/2.0/BarramundiFish/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Boom Box](assets/gltf_samples/2.0/BoomBox)|![](assets/gltf_samples/2.0/BoomBox/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Boom Box With Axes](assets/gltf_samples/2.0/BoomBoxWithAxes)|![](assets/gltf_samples/2.0/BoomBoxWithAxes/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Box](assets/gltf_samples/2.0/Box)|![](assets/gltf_samples/2.0/Box/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Box  With  Spaces](assets/gltf_samples/2.0/Box With Spaces)|![](assets/gltf_samples/2.0/Box With Spaces/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Box Animated](assets/gltf_samples/2.0/BoxAnimated)|![](assets/gltf_samples/2.0/BoxAnimated/screenshot/screenshot.gif)|:x:| | |
|[Box Interleaved](assets/gltf_samples/2.0/BoxInterleaved)|![](assets/gltf_samples/2.0/BoxInterleaved/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Box Textured](assets/gltf_samples/2.0/BoxTextured)|![](assets/gltf_samples/2.0/BoxTextured/screenshot/screenshot.png)|:hourglass:| | |
|[Box Textured Non Power Of Two](assets/gltf_samples/2.0/BoxTexturedNonPowerOfTwo)|![](assets/gltf_samples/2.0/BoxTexturedNonPowerOfTwo/screenshot/screenshot.png)|:hourglass:| | |
|[Box Vertex Colors](assets/gltf_samples/2.0/BoxVertexColors)|![](assets/gltf_samples/2.0/BoxVertexColors/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Brain Stem](assets/gltf_samples/2.0/BrainStem)|![](assets/gltf_samples/2.0/BrainStem/screenshot/screenshot.gif)|:x:| | |
|[Buggy](assets/gltf_samples/2.0/Buggy)|![](assets/gltf_samples/2.0/Buggy/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Cameras](assets/gltf_samples/2.0/Cameras)|![](assets/gltf_samples/2.0/Cameras/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Cesium Man](assets/gltf_samples/2.0/CesiumMan)|![](assets/gltf_samples/2.0/CesiumMan/screenshot/screenshot.gif)|:hourglass:| | |
|[Cesium Milk Truck](assets/gltf_samples/2.0/CesiumMilkTruck)|![](assets/gltf_samples/2.0/CesiumMilkTruck/screenshot/screenshot.gif)|:heavy_check_mark:| | |
|[Clear Coat Test](assets/gltf_samples/2.0/ClearCoatTest)|![](assets/gltf_samples/2.0/ClearCoatTest/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Corset](assets/gltf_samples/2.0/Corset)|![](assets/gltf_samples/2.0/Corset/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Cube](assets/gltf_samples/2.0/Cube)|![](assets/gltf_samples/2.0/Cube/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Damaged Helmet](assets/gltf_samples/2.0/DamagedHelmet)|![](assets/gltf_samples/2.0/DamagedHelmet/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Duck](assets/gltf_samples/2.0/Duck)|![](assets/gltf_samples/2.0/Duck/screenshot/screenshot.png)|:hourglass:| | |
|[Environment Test](assets/gltf_samples/2.0/EnvironmentTest)|![](assets/gltf_samples/2.0/EnvironmentTest/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Flight Helmet](assets/gltf_samples/2.0/FlightHelmet)|![](assets/gltf_samples/2.0/FlightHelmet/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Fox](assets/gltf_samples/2.0/Fox)|![](assets/gltf_samples/2.0/Fox/screenshot/screenshot.jpg)|:hourglass:| | |
|[Gearbox Assy](assets/gltf_samples/2.0/GearboxAssy)|![](assets/gltf_samples/2.0/GearboxAssy/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Interpolation Test](assets/gltf_samples/2.0/InterpolationTest)|![](assets/gltf_samples/2.0/InterpolationTest/screenshot/screenshot.gif)|:x:| | |
|[Lantern](assets/gltf_samples/2.0/Lantern)|![](assets/gltf_samples/2.0/Lantern/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Materials Variants Shoe](assets/gltf_samples/2.0/MaterialsVariantsShoe)|![](assets/gltf_samples/2.0/MaterialsVariantsShoe/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Metal Rough Spheres](assets/gltf_samples/2.0/MetalRoughSpheres)|![](assets/gltf_samples/2.0/MetalRoughSpheres/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Metal Rough Spheres No Textures](assets/gltf_samples/2.0/MetalRoughSpheresNoTextures)|![](assets/gltf_samples/2.0/MetalRoughSpheresNoTextures/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Morph Primitives Test](assets/gltf_samples/2.0/MorphPrimitivesTest)|![](assets/gltf_samples/2.0/MorphPrimitivesTest/screenshot/screenshot.jpg)|:hourglass:| | |
|[Morph Stress Test](assets/gltf_samples/2.0/MorphStressTest)|![](assets/gltf_samples/2.0/MorphStressTest/screenshot/screenshot.jpg)|:x:| | |
|[Multi UVTest](assets/gltf_samples/2.0/MultiUVTest)|![](assets/gltf_samples/2.0/MultiUVTest/screenshot/screenshot.jpg)|:heavy_check_mark:| | |
|[Normal Tangent Mirror Test](assets/gltf_samples/2.0/NormalTangentMirrorTest)|![](assets/gltf_samples/2.0/NormalTangentMirrorTest/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Normal Tangent Test](assets/gltf_samples/2.0/NormalTangentTest)|![](assets/gltf_samples/2.0/NormalTangentTest/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Orientation Test](assets/gltf_samples/2.0/OrientationTest)|![](assets/gltf_samples/2.0/OrientationTest/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Reciprocating Saw](assets/gltf_samples/2.0/ReciprocatingSaw)|![](assets/gltf_samples/2.0/ReciprocatingSaw/screenshot/screenshot.png)|:heavy_check_mark:| | |
|[Recursive Skeletons](assets/gltf_samples/2.0/RecursiveSkeletons)|![](assets/gltf_samples/2.0/RecursiveSkeletons/screenshot/screenshot.jpg)|:x:| | |
