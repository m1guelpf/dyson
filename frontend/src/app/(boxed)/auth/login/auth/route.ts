import { NextRequest, NextResponse } from 'next/server'

export const runtime = 'edge'

export const POST = async (req: NextRequest): Promise<Response> => {
	return NextResponse.json({})
}
