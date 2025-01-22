fn nuOptionsFunc(){
	let nuOptions = 0
	let RougePosX = -0.5999
	let RougePosY = 0.1599
	if RougePosX == -0.5999 & RougePosY == 0.1599{
		RougePosY -= RougePosY + 0.0533
	}
	else if RougePosX == -0.5999 & RougePosY == 0.0533{
		if nuOptions == 0{
			*nuOptions+=1;
		}
		else if nuOptions == 1{
			RougePosY -= RougePosY + 0.4798
			if RougePosX == -0.5999 & RougePosY == 0.4798{
				let Vibration  = 0  // Off 
				let NuSurround = 0  // Off
				let NuSoundVol = 10 // 10/10
				let NuMusicVol = 10 // 10/10
				let NuVoiceVol = 10 // 10/10
				let NuSubtitle = 1  // On
				let NuHor      = 0  // 0
				let NuVer      = 0  // 0
				let NuSwapCtrl = 0  // No
				
				if Vibration == 0{
					*Vibration +=1;
					RougePosY -= RougePosY + 0.3732
					if RougePosX == -0.5999 & RougePosY == 0.3732
					{
						if NuSurround == 0{
							nuloop_() // audio loops surround sound on from a example func
							*NuSurround += 1;
							RougePosY -= RougePosY + 0.2665
							if RougePosX == -0.5999 & RougePosY == 0.2665
							{
								
								if NuSoundVol == 10{
									NuSoundVol -= 1 // left
									RougePosY -= RougePosY + 0.1599
									if RougePosX == -0.5999 & RougePosY == 0.1599
										{
											if NuMusicVol == 10{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 9{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 8{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 7{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 6{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 5{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 4{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 3{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 2{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 1{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 0{
											}
										}
									}
									if NuSoundVol == 9{
										NuSoundVol += 1 // right
										RougePosY -= RougePosY + 0.1599
										if RougePosX == -0.5999 & RougePosY == 0.1599
										{
											if NuMusicVol == 10{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 9{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 8{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 7{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 6{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 5{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 4{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 3{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 2{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 1{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 0{
											}
										}
								}
								else if NuSoundVol == 9{
									NuSoundVol -= 1 // left
									RougePosY -= RougePosY + 0.1599
									if RougePosX == -0.5999 & RougePosY == 0.1599
										{
											if NuMusicVol == 10{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 9{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 8{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 7{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 6{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 5{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 4{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 3{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 2{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 1{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 0{
											}
										}
									if NuSoundVol == 8{
										NuSoundVol += 1 // right
										RougePosY -= RougePosY + 0.1599
										if RougePosX == -0.5999 & RougePosY == 0.1599
										{
											if NuMusicVol == 10{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 9{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 8{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 7{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 6{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 5{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 4{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 3{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 2{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 1{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 0{
											}
										}
									}
								}
								else if NuSoundVol == 8{
									NuSoundVol -= 1 // left
									RougePosY -= RougePosY + 0.1599
									if RougePosX == -0.5999 & RougePosY == 0.1599
									{
										if NuMusicVol == 10{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 9{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 8{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 7{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 6{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 5{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 4{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 3{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 2{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 1{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 0{
										}
									}
								}
									if NuSoundVol == 7{
										NuSoundVol += 1 // right
										RougePosY -= RougePosY + 0.1599
										if RougePosX == -0.5999 & RougePosY == 0.1599
										{
											if NuMusicVol == 10{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 9{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 8{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 7{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 6{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 5{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 4{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 3{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 2{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 1{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 0{
											}
										}
									}
								}
								else if NuSoundVol == 7{
									NuSoundVol -= 1 // left
									RougePosY -= RougePosY + 0.1599
									if RougePosX == -0.5999 & RougePosY == 0.1599
									{
										if NuMusicVol == 10{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 9{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 8{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 7{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 6{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 5{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 4{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 3{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 2{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 1{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 0{
										}
									}
									if NuSoundVol == 6{
										NuSoundVol += 1 // right
										RougePosY -= RougePosY + 0.1599
										if RougePosX == -0.5999 & RougePosY == 0.1599
										{
											if NuMusicVol == 10{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 9{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 8{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 7{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 6{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 5{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 4{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 3{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 2{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 1{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 0{
											}
										}
									}
								}
								else if NuSoundVol == 6{
									NuSoundVol -= 1 // left
									RougePosY -= RougePosY + 0.1599
									if RougePosX == -0.5999 & RougePosY == 0.1599
									{
										if NuMusicVol == 10{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 9{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 8{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 7{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 6{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 5{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 4{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 3{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 2{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 1{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 0{
										}
									}
									if NuSoundVol == 5{
										NuSoundVol += 1 // right
										RougePosY -= RougePosY + 0.1599
										if RougePosX == -0.5999 & RougePosY == 0.1599
										{
											if NuMusicVol == 10{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 9{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 8{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 7{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 6{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 5{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 4{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 3{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 2{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 1{
												NuMusicVol -= 1
											}
											else if NuMusicVol == 0{
											}
										}
									}
								}
								else if NuSoundVol == 5{
									NuSoundVol -= 1 // left
									RougePosY -= RougePosY + 0.1599
									if RougePosX == -0.5999 & RougePosY == 0.1599
									{
										if NuMusicVol == 10{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 9{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 8{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 7{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 6{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 5{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 4{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 3{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 2{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 1{
											NuMusicVol -= 1
										}
										else if NuMusicVol == 0{
										}
									}
									if NuSoundVol == 4{
										NuSoundVol += 1 // right
									}
								}
								else if NuSoundVol == 4{
									NuSoundVol -= 1 // left
									if NuSoundVol == 3{
										NuSoundVol += 1 // right
									}
								}
								else if NuSoundVol == 3{
									NuSoundVol -= 1 // left
									if NuSoundVol == 2{
										NuSoundVol += 1 // right
									}
								}
								else if NuSoundVol == 2{
									NuSoundVol -= 1 // left
									if NuSoundVol == 1{
										NuSoundVol += 1 // right
									}
								}
								else if NuSoundVol == 1{
									NuSoundVol -= 1 // left
									if NuSoundVol == 0{
										NuSoundVol += 1 // right
									}
								}
								else if NuSoundVol == 0{
								}
							}
							
						}
						else if NuSurround == 1{
							nuloop_() // audio loops surround sound off from a example func
							*NuSurround -= 1;
						
						}
						
					}
				}
				else if Vibration == 1{
					nuRumble_() // makes a rumble sound on your controller on by pressing x on the vibration
					*Vibration-=1;
					RougePosY -= RougePosY + 0.3732
					if RougePosX == -0.6000 & RougePosY == 0.3732
					{
						if NuSurround == 0{
							nuloop_() // audio loops surround sound on from a example func
							*NuSurround += 1;
						}
						else if NuSurround == 1{
							nuloop_() // audio loops surround sound off from a example func
							*NuSurround -= 1;
						
						}
						
					}
				}
			}
		}
	}
};
				
