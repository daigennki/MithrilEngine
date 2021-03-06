cbuffer renderParams : register(b0)
{
	float4x4 proj;	// projection matrix
    float4x4 view;	// view matrix
    float4x4 invProjView;
    float3 camera_pos;
};
struct VS_OUTPUT
{
    float4 position : SV_POSITION;
    float3 frag_pos : POSITION; //vertex position to fragment shader
};
VS_OUTPUT main(float3 position : POSITION)
{
    VS_OUTPUT output;
    output.frag_pos = position;
	float4 new_pos = mul(proj, float4(mul(float3x3(view[0].xyz, view[1].xyz, view[2].xyz), position), 1.0));
	output.position = new_pos.xyww;
    return output;
}