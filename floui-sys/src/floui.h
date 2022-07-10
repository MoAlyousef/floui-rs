#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void Cfloui_log(const char *s);

#define DECLARE_SHARED_METHODS(x)                                                                  \
    void x##_id(x *self, const char *id);                                                          \
    void x##_background(x *self, unsigned int col);                                                \
    void x##_size(x *self, int w, int h);                                                          \
    void *x##_inner(x *self);

#define DECLARE_SHARED_GROUP_METHODS(x)                                                            \
    void x##_add(x *self, const CWidget *w);                                                       \
    void x##_remove(x *self, const CWidget *w);                                                    \
    void x##_clear(x *self);                                                                       \
    void x##_spacing(x *self, int spacing);

typedef struct CFlouiViewController CFlouiViewController;

CFlouiViewController *CFlouiViewController_new(void *, void *, void *);

void CFlouiViewController_handle_events(void *);

typedef struct CWidget CWidget;

DECLARE_SHARED_METHODS(CWidget);

typedef void (*CFlouiCallback)(CWidget *, void *data);

typedef struct CMainView CMainView;

CMainView *CMainView_new(const CFlouiViewController *fvc);

DECLARE_SHARED_METHODS(CMainView);

DECLARE_SHARED_GROUP_METHODS(CMainView);

typedef struct CVStack CVStack;

CVStack *CVStack_new(void);

DECLARE_SHARED_METHODS(CVStack);

DECLARE_SHARED_GROUP_METHODS(CVStack);

typedef struct CHStack CHStack;

CHStack *CHStack_new(void);

DECLARE_SHARED_METHODS(CHStack);

DECLARE_SHARED_GROUP_METHODS(CHStack);

typedef struct CButton CButton;

DECLARE_SHARED_METHODS(CButton);

CButton *CButton_new(const char *label);

void CButton_action(CButton *self, CFlouiCallback cb, void *data);

void CButton_filled(CButton *self);

void CButton_foreground(CButton *self, unsigned int c);

typedef struct CText CText;

DECLARE_SHARED_METHODS(CText);

CText *CText_new(const char *label);

void CText_text(CText *self, const char *t);

void CText_center(CText *self);

void CText_bold(CText *self);

void CText_foreground(CText *self, unsigned int c);

void CText_fontsize(CText *self, int size);

typedef struct CTextField CTextField;

DECLARE_SHARED_METHODS(CTextField);

CTextField *CTextField_new();

void CTextField_text(CTextField *self, const char *t);

void CTextField_center(CTextField *self);

void CTextField_foreground(CTextField *self, unsigned int c);

void CTextField_fontsize(CTextField *self, int size);

typedef struct CSpacer CSpacer;

DECLARE_SHARED_METHODS(CSpacer);

CSpacer *CSpacer_new();

#ifdef __cplusplus
}
#endif