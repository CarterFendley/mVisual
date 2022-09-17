/*
Can do the color transformations here becuase we do not have meshes.
See: https://gamedev.stackexchange.com/a/139061

The color calculations simplified Phong model. It excludes specular reflection and any difference in ambient vs diffuse material color.
*/
pub const SHADER: &str = r#"
    // Lighting settings
    uniform vec3 uAmbientLightColor;
    uniform vec3 uDiffuseLightPosition;
    uniform vec3 uDiffuseLightColor;

    uniform vec3 uMaterialColor;

    // Transformations
    uniform mat4 uModelView;
    uniform mat4 uModelViewProjection;

    // Vertex data
    attribute vec3 aVertexPosition;
    attribute vec3 aVertexNormal;

    varying lowp vec4 vColor;

    void main() {
        // Calculate the final position
        gl_Position = uModelViewProjection * vec4(aVertexPosition, 1.0);

        /*
        Calculate the color

        The 0.0 in `transformedNormal` is important to not apply any of the translations from the model + view transformations. See the "One last thing" section of this page: https://learnopengl.com/Lighting/Basic-Lighting

        That page also notes if, in the future there is a non-uniform scalling applied (scales x,y, or z differently than the others), it will become an issue and require us to no longer use the MV for transforming normals.
        */
        vec3 transformedNormal = normalize(vec3(uModelView * vec4(aVertexNormal, 0.0)));
        vec3 diffuseNormal = normalize(uDiffuseLightPosition.xyz);
        float flooredDotProduct = max(dot(transformedNormal, diffuseNormal), 0.0);

        vec3 vertexColor = uAmbientLightColor * uMaterialColor;
        vertexColor += flooredDotProduct * uDiffuseLightColor * uMaterialColor;

        vColor = vec4(vertexColor, 1.0);
    }
"#;
