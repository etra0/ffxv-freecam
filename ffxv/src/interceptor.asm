
.data
PUBLIC get_camera_data
PUBLIC get_camera_data_end

.code
; RDI + 3820 focus position
get_camera_data PROC
  
  push rax
  lea rax, [get_camera_data]

  ; backup xmm0 and xmm1
  movaps [rax + 100h], xmm0
  movaps [rax + 110h], xmm1

  pushf
  cmp DWORD PTR [rax + 1F0h], 1
  jne not_active

  movaps xmm0, [rax + 200h]
  movaps xmm1, [rax + 220h]
  movaps [rdi + 3820h], xmm0
  movaps [rdi + 3830h], xmm1

  jmp ending


  not_active:
  movaps xmm0, [rdi + 3820h]
  movaps xmm1, [rdi + 3830h]
  movaps [rax + 200h], xmm0
  movaps [rax + 220h], xmm1

  ending:
  popf
  ; load the xmm{0, 1} backup
  movaps xmm0, [rax + 100h]
  movaps xmm1, [rax + 110h]
  pop rax

  ;original code
  lea rdx, [rbp + 67]
  mov rcx, rdi

  ret
get_camera_data_end::
get_camera_data ENDP

END
