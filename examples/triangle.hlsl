struct PSInput
{
	float4 position : SV_POSITION;
	float4 color : COLOR;
};

PSInput VSMain(float3 pos: POS, float4 color: COLOR) {
    PSInput result;

    result.position = float4(pos, 1.0);
    result.color = color;
    return result;
}

float4 PSMain(PSInput psi): SV_TARGET {
    return psi.color;
}
