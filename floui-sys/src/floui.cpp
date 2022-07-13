#include "floui.h"

#define FLOUI_IMPL
#define FLOUI_IOS_WEBVIEW
#include "../floui/floui.hpp"

void Cfloui_log(const char *s) { floui_log(s); }

#define DEFINE_SHARED_METHODS(x)                                                                   \
    void x##_id(x *self, const char *id) { ((Widget *)self)->id(id); }                             \
    void x##_background(x *self, unsigned int col) { ((Widget *)self)->background(col); }          \
    void x##_size(x *self, int w, int h) { ((Widget *)self)->size(w, h); }                         \
    void *x##_inner(x *self) { return ((Widget *)self)->inner(); }

#define DEFINE_SHARED_GROUP_METHODS(x)                                                             \
    void x##_add(x *self, const CWidget *w) { ((VStack *)self)->add(*(Widget *)w); }               \
    void x##_remove(x *self, const CWidget *w) { ((VStack *)self)->remove(*(Widget *)w); }         \
    void x##_clear(x *self) { ((VStack *)self)->clear(); }                                         \
    void x##_spacing(x *self, int spacing) { ((VStack *)self)->spacing(spacing); }

CFlouiViewController *CFlouiViewController_new(void *arg1, void *arg2, void *arg3) {
    return (CFlouiViewController *)new FlouiViewController(arg1, arg2, arg3);
}

void CFlouiViewController_handle_events(void *view) { FlouiViewController::handle_events(view); }

DEFINE_SHARED_METHODS(CWidget)

CMainView *CMainView_new(const CFlouiViewController *fvc) {
    return (CMainView *)new MainView(*(FlouiViewController *)fvc, {});
}

DEFINE_SHARED_METHODS(CMainView)

DEFINE_SHARED_GROUP_METHODS(CMainView)

CVStack *CVStack_new(void) { return (CVStack *)new VStack({}); }

DEFINE_SHARED_METHODS(CVStack)

DEFINE_SHARED_GROUP_METHODS(CVStack)

CHStack *CHStack_new(void) { return (CHStack *)new HStack({}); }

DEFINE_SHARED_METHODS(CHStack)

DEFINE_SHARED_GROUP_METHODS(CHStack)

CButton *CButton_new(const char *label) { return (CButton *)new Button(label); }

DEFINE_SHARED_METHODS(CButton)

void CButton_action(CButton *self, CFlouiCallback cb, void *data) {
    ((Button *)self)->action([=](Widget &) { (*cb)((CWidget *)self, data); });
}

void CButton_filled(CButton *self) { ((Button *)self)->filled(); }

void CButton_foreground(CButton *self, unsigned int c) { ((Button *)self)->foreground(c); }

DEFINE_SHARED_METHODS(CToggle)

CToggle *CToggle_new(const char *label) { return (CToggle *)new Toggle(label); }

void CToggle_action(CToggle *self, CFlouiCallback cb, void *data) {
    ((Toggle *)self)->action([=](Widget &) { (*cb)((CWidget *)self, data); });
}

void CToggle_set_value(CToggle *self, int val) { ((Toggle *)self)->value(val); }

int CToggle_value(CToggle *self) { return ((Toggle *)self)->value(); }

void CToggle_foreground(CToggle *self, unsigned int c) { ((Toggle *)self)->foreground(c); }

DEFINE_SHARED_METHODS(CCheck)

CCheck *CCheck_new(const char *label) { return (CCheck *)new Check(label); }

void CCheck_action(CCheck *self, CFlouiCallback cb, void *data) {
    ((Check *)self)->action([=](Widget &) { (*cb)((CWidget *)self, data); });
}

void CCheck_set_value(CCheck *self, int val) { ((Check *)self)->value(val); }

int CCheck_value(CCheck *self) { return ((Check *)self)->value(); }

void CCheck_foreground(CCheck *self, unsigned int c) { ((Check *)self)->foreground(c); }

DEFINE_SHARED_METHODS(CSlider)

CSlider *CSlider_new() { return (CSlider *)new Slider(); }

void CSlider_action(CSlider *self, CFlouiCallback cb, void *data) {
    ((Slider *)self)->action([=](Widget &) { (*cb)((CWidget *)self, data); });
}

void CSlider_set_value(CSlider *self, double val) { ((Slider *)self)->value(val); }

double CSlider_value(CSlider *self) { return ((Slider *)self)->value(); }

void CSlider_foreground(CSlider *self, unsigned int c) { ((Slider *)self)->foreground(c); }

CText *CText_new(const char *label) { return (CText *)new Text(label); }

DEFINE_SHARED_METHODS(CText)

void CText_text(CText *self, const char *t) { ((Text *)self)->text(t); }

void CText_center(CText *self) { ((Text *)self)->center(); }

void CText_bold(CText *self) { ((Text *)self)->bold(); }

void CText_foreground(CText *self, unsigned int c) { ((Text *)self)->foreground(c); }

void CText_fontsize(CText *self, int size) { ((Text *)self)->fontsize(size); }

CTextField *CTextField_new() { return (CTextField *)new TextField(); }

DEFINE_SHARED_METHODS(CTextField)

void CTextField_text(CTextField *self, const char *t) { ((TextField *)self)->text(t); }

void CTextField_center(CTextField *self) { ((TextField *)self)->center(); }

void CTextField_foreground(CTextField *self, unsigned int c) { ((TextField *)self)->foreground(c); }

void CTextField_fontsize(CTextField *self, int size) { ((TextField *)self)->fontsize(size); }

CSpacer *CSpacer_new() { return (CSpacer *)new Spacer(); }

DEFINE_SHARED_METHODS(CSpacer)

DEFINE_SHARED_METHODS(CImageView)

CImageView *CImageView_new() { return (CImageView *)new ImageView(); }

CImageView *CImageView_load(const char *path) { return (CImageView *)new ImageView(path); }

void CImageView_image(CImageView *self, const char *path) { ((ImageView *)self)->image(path); }

DEFINE_SHARED_METHODS(CWebView)

CWebView *CWebView_new() { return (CWebView *)new WebView(); }

void CWebView_load_url(CWebView *self, const char *url) { ((WebView *)self)->load_url(url); }

void CWebView_load_html(CWebView *self, const char *html) { ((WebView *)self)->load_html(html); }

DEFINE_SHARED_METHODS(CScrollView)

CScrollView *CScrollView_new(const CWidget *w) {
    return (CScrollView *)new ScrollView(*(Widget *)w);
}